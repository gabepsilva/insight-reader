//! Single-instance enforcement to prevent multiple instances from running simultaneously

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use dirs::config_dir;
use fs2::FileExt;
use thiserror::Error;
use tracing::{debug, error, info, warn};

// Global reference to the instance guard for IPC polling
static INSTANCE_GUARD: Mutex<Option<Arc<SingleInstanceGuard>>> = Mutex::new(None);

#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};

#[cfg(windows)]
use std::os::windows::net::NamedPipeListener;

const APP_CONFIG_DIR_NAME: &str = "insight-reader";
const LOCK_FILE_NAME: &str = ".lock";
const IPC_MESSAGE_BRING_TO_FRONT: &[u8] = b"BRING_TO_FRONT";

/// Error type for single-instance operations
#[derive(Error, Debug)]
pub enum SingleInstanceError {
    /// Another instance is already running
    #[error("Another instance of Insight Reader is already running")]
    LockFailed,
    
    /// File system error (permissions, etc.)
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

/// Guard that holds the lock file. The lock is released when this guard is dropped.
pub struct SingleInstanceGuard {
    _file: File,
    #[cfg(unix)]
    listener: Option<UnixListener>,
    #[cfg(windows)]
    listener: Option<NamedPipeListener>,
}

impl SingleInstanceGuard {
    #[cfg(unix)]
    fn new(file: File, listener: Option<UnixListener>) -> Self {
        Self { _file: file, listener }
    }
    
    #[cfg(windows)]
    fn new(file: File, listener: Option<NamedPipeListener>) -> Self {
        Self { _file: file, listener }
    }
}

// Helper to get config directory or return appropriate error
fn get_config_dir() -> Result<PathBuf, io::Error> {
    config_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Config directory not found",
        )
    })
}

/// Attempts to acquire an exclusive lock on the lock file.
/// 
/// Returns `Ok(Arc<SingleInstanceGuard>)` if the lock was acquired successfully,
/// or `Err(SingleInstanceError::LockFailed)` if another instance is already running.
/// 
/// The lock file is created in the same directory as the config file:
/// `{config_dir}/insight-reader/.lock`
pub fn try_lock() -> Result<Arc<SingleInstanceGuard>, SingleInstanceError> {
    let lock_path = lock_file_path()?;
    
    // Ensure the config directory exists
    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Open or create the lock file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&lock_path)
        .map_err(SingleInstanceError::IoError)?;
    
    // Try to acquire an exclusive lock (non-blocking)
    match file.try_lock_exclusive() {
        Ok(()) => {
            info!(lock_path = %lock_path.display(), "Single-instance lock acquired");
            
            // Start IPC server to listen for bring-to-front messages
            let listener = start_ipc_server().ok();
            if listener.is_some() {
                info!("IPC server started for bring-to-front messages");
            } else {
                warn!("Failed to start IPC server, continuing without it");
            }
            
            let guard = SingleInstanceGuard::new(file, listener);
            let guard_arc = Arc::new(guard);
            
            // Store globally for IPC polling
            *INSTANCE_GUARD.lock().unwrap() = Some(guard_arc.clone());
            
            Ok(guard_arc)
        }
        Err(e) => {
            // Lock failed - another instance is running
            // Try to send bring-to-front message to existing instance
            let _ = send_bring_to_front_message().inspect_err(|ipc_err| {
                warn!(error = %ipc_err, "Failed to send bring-to-front message to existing instance");
            });
            error!(error = %e, lock_path = %lock_path.display(), "Failed to acquire lock");
            Err(SingleInstanceError::LockFailed)
        }
    }
}

#[cfg(unix)]
fn start_ipc_server() -> Result<UnixListener, io::Error> {
    let socket_path = ipc_socket_path()?;
    
    // Remove existing socket file if it exists (stale from previous crash)
    if socket_path.exists() {
        let _ = std::fs::remove_file(&socket_path);
    }
    
    // Ensure the directory exists
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let listener = UnixListener::bind(&socket_path)?;
    listener.set_nonblocking(true)?;
    
    info!(socket_path = %socket_path.display(), "IPC server listening on Unix socket");
    Ok(listener)
}

#[cfg(windows)]
fn start_ipc_server() -> Result<NamedPipeListener, io::Error> {
    let pipe_name = ipc_pipe_name()?;
    // Windows named pipes are more complex - for now, return an error
    // TODO: Implement proper Windows named pipe server
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Windows named pipe IPC not yet implemented",
    ))
}

#[cfg(unix)]
fn send_bring_to_front_message() -> Result<(), io::Error> {
    let socket_path = ipc_socket_path()?;
    
    if !socket_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "IPC socket not found",
        ));
    }
    
    let mut stream = UnixStream::connect(&socket_path)?;
    stream.write_all(IPC_MESSAGE_BRING_TO_FRONT)?;
    stream.flush()?;
    info!("Sent bring-to-front message to existing instance");
    Ok(())
}

#[cfg(windows)]
fn send_bring_to_front_message() -> Result<(), io::Error> {
    // TODO: Implement Windows named pipe client
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Windows named pipe IPC not yet implemented",
    ))
}

#[cfg(unix)]
fn ipc_socket_path() -> Result<PathBuf, io::Error> {
    Ok(get_config_dir()?.join(APP_CONFIG_DIR_NAME).join("ipc.sock"))
}

#[cfg(windows)]
fn ipc_pipe_name() -> Result<String, io::Error> {
    let config_dir = get_config_dir()?;
    // Windows named pipe format: \\.\pipe\name
    let pipe_name = format!(
        r"\\.\pipe\insight-reader-{}",
        config_dir.to_string_lossy().replace('\\', "-").replace(':', "")
    );
    Ok(pipe_name)
}

fn lock_file_path() -> Result<PathBuf, SingleInstanceError> {
    Ok(get_config_dir()
        .map_err(|e| SingleInstanceError::IoError(e))?
        .join(APP_CONFIG_DIR_NAME)
        .join(LOCK_FILE_NAME))
}

/// Try to receive a bring-to-front message from a new instance (non-blocking)
/// This function polls the global instance guard for IPC messages.
pub fn try_recv_bring_to_front() -> bool {
    let guard = INSTANCE_GUARD.lock().unwrap();
    let Some(guard) = guard.as_ref() else {
        return false;
    };
    
    #[cfg(unix)]
    {
        let Some(listener) = &guard.listener else {
            return false;
        };
        
        // Accept any pending connections (non-blocking)
        let Ok((mut stream, _addr)) = listener.accept() else {
            return false;
        };
        
        let mut buf = [0u8; 32];
        match stream.read(&mut buf) {
            Ok(n) if n >= IPC_MESSAGE_BRING_TO_FRONT.len() 
                && &buf[..IPC_MESSAGE_BRING_TO_FRONT.len()] == IPC_MESSAGE_BRING_TO_FRONT => {
                info!("Received bring-to-front message from new instance");
                return true;
            }
            Ok(_) => {
                // Message too short or doesn't match, ignore
            }
            Err(e) => {
                debug!(error = %e, "Error reading IPC message");
            }
        }
    }
    
    #[cfg(windows)]
    {
        // TODO: Implement Windows named pipe polling
        let _ = guard;
    }
    
    false
}
