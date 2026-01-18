//! Command-line interface for Insight Reader

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "insight-reader")]
#[command(version, about = "Insight Reader - Text-to-Speech Application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Log file management
    Logs {
        #[command(subcommand)]
        action: LogsAction,
    },
    /// Show status of required system permissions (macOS only)
    Permissions,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show the current configuration
    Show,
}

#[derive(Subcommand)]
pub enum LogsAction {
    /// Show the latest log file (last 50 lines)
    Show {
        /// Number of lines to show (default: 50)
        #[arg(short = 'n', long, default_value = "50")]
        lines: usize,
    },
}

/// Initialize logging for CLI mode (file only, respects RUST_LOG)
pub fn init_logging() {
    use crate::config;
    use crate::logging;

    let log_config = logging::LoggingConfig {
        verbosity: config::load_log_level(),
        log_to_stderr: false, // CLI only logs to file
        log_to_file: true,
        log_dir: None, // Uses default log directory
    };

    // Ignore errors - CLI can work without logging
    let _ = logging::init_logging(&log_config);
}

/// Check if the current arguments indicate CLI mode
/// Returns true if any arguments are provided (GUI takes no arguments)
///
/// This allows clap to handle all argument parsing, including showing
/// help/errors for invalid arguments. The GUI mode only runs when no
/// arguments are provided.
pub fn is_cli_mode() -> bool {
    std::env::args().nth(1).is_some()
}

/// Run the CLI interface
/// This will parse arguments and show help/errors via clap if invalid
pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // No subcommand provided, show help
            println!("No command specified. Use --help for available commands.");
            std::process::exit(0);
        }
        Some(Commands::Config {
            action: ConfigAction::Show,
        }) => show_config(),
        Some(Commands::Logs {
            action: LogsAction::Show { lines },
        }) => show_logs(lines),
        Some(Commands::Permissions) => show_permissions(),
    }
}

/// Exit with an error message
fn exit_with_error(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn show_config() {
    use dirs::config_dir;
    use std::fs;

    const MAX_CONFIG_SIZE: u64 = 10 * 1024 * 1024; // 10MB limit

    let Some(config_dir) = config_dir() else {
        eprintln!("Error: Could not determine config directory");
        std::process::exit(1);
    };

    let config_path = config_dir.join("insight-reader").join("config.json");

    if !config_path.exists() {
        println!("Config file does not exist: {}", config_path.display());
        println!("(This is normal if you haven't launched the GUI yet)");
        return;
    }

    // Check file size before reading
    let metadata = fs::metadata(&config_path)
        .unwrap_or_else(|e| exit_with_error(&format!("Error reading config file metadata: {}", e)));

    if metadata.len() > MAX_CONFIG_SIZE {
        eprintln!(
            "Error: Config file is too large ({} bytes, max {} bytes)",
            metadata.len(),
            MAX_CONFIG_SIZE
        );
        std::process::exit(1);
    }

    let contents = fs::read_to_string(&config_path)
        .unwrap_or_else(|e| exit_with_error(&format!("Error reading config file: {}", e)));

    println!("Config file: {}", config_path.display());
    println!();
    println!("{}", contents);
}

fn show_logs(lines: usize) {
    use crate::logging;
    use std::fs;
    use std::io::{BufRead, BufReader};

    let log_dir = logging::default_log_dir();

    if !log_dir.exists() {
        println!("Log directory does not exist: {}", log_dir.display());
        println!("(This is normal if no logging has occurred yet)");
        return;
    }

    // Find the latest log file (daily rotation creates files like insight-reader.log, insight-reader.log.2026-01-06, etc.)
    let latest_log = fs::read_dir(&log_dir)
        .unwrap_or_else(|e| {
            exit_with_error(&format!(
                "Error reading log directory {}: {}",
                log_dir.display(),
                e
            ))
        })
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let file_name = path.file_name()?.to_string_lossy();
            if path.is_file() && file_name.starts_with("insight-reader.log") {
                let modified = entry.metadata().ok()?.modified().ok()?;
                Some((path, modified))
            } else {
                None
            }
        })
        .max_by_key(|(_, modified)| *modified)
        .map(|(path, _)| path);

    let Some(latest_log) = latest_log else {
        println!("No log files found in: {}", log_dir.display());
        return;
    };

    // Read the file and get the last N lines
    let file = fs::File::open(&latest_log)
        .unwrap_or_else(|e| exit_with_error(&format!("Error opening log file: {}", e)));

    let reader = BufReader::new(file);
    let all_lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap_or_else(|e| exit_with_error(&format!("Error reading log file: {}", e)));

    let start = all_lines.len().saturating_sub(lines);
    let lines_to_show = &all_lines[start..];

    // Print log lines first
    for line in lines_to_show {
        println!("{}", line);
    }

    // Print summary at the end
    println!("Log file: {}", latest_log.display());
    println!("Showing last {} line(s)", lines_to_show.len());
}

fn show_permissions() {
    #[cfg(target_os = "macos")]
    {
        show_permissions_macos();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("Permission checking is only available on macOS.");
        println!();
        println!("On Linux and Windows, permissions are typically handled differently:");
        println!("  - Linux: No special permissions needed for clipboard or screenshots");
        println!("  - Windows: No special permissions needed for clipboard or screenshots");
    }
}

#[cfg(target_os = "macos")]
fn show_permissions_macos() {
    use macos_accessibility_client::accessibility::application_is_trusted;

    println!("Insight Reader - Permission Status (macOS)");
    println!("==========================================");
    println!();

    // Check Accessibility permission
    let accessibility_granted = application_is_trusted();
    let accessibility_status = if accessibility_granted {
        "✓ Granted"
    } else {
        "✗ Not Granted"
    };
    println!("Accessibility:    {}", accessibility_status);
    println!("  Required for:   Capturing selected text (simulates Cmd+C)");
    println!("  Settings path:  System Settings > Privacy & Security > Accessibility");
    println!();

    // Check Screen Recording permission
    let screen_recording_granted = check_screen_recording_permission();
    let screen_recording_status = if screen_recording_granted {
        "✓ Granted"
    } else {
        "✗ Not Granted"
    };
    println!("Screen Recording: {}", screen_recording_status);
    println!("  Required for:   Screenshot OCR feature");
    println!("  Settings path:  System Settings > Privacy & Security > Screen Recording");
    println!();

    // Summary
    let all_granted = accessibility_granted && screen_recording_granted;
    if all_granted {
        println!("All required permissions are granted.");
    } else {
        println!("Some permissions are missing. The app may not function correctly.");
        println!();
        println!("To grant permissions:");
        println!("  1. Open System Settings");
        println!("  2. Go to Privacy & Security");
        println!("  3. Enable Insight Reader for each required permission");
    }
}

/// Check Screen Recording permission using CoreGraphics API
#[cfg(target_os = "macos")]
fn check_screen_recording_permission() -> bool {
    // CGPreflightScreenCaptureAccess() returns true if the app has screen recording permission
    // This function does NOT prompt the user - it just checks the current status
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGPreflightScreenCaptureAccess() -> bool;
    }

    // Safety: CGPreflightScreenCaptureAccess is a safe C function with no side effects
    unsafe { CGPreflightScreenCaptureAccess() }
}
