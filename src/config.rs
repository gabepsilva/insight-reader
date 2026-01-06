//! Persistent configuration handling (mirrors the original grafl config).
//!
//! Persists the selected voice provider and log level.
//! The on-disk format is compatible with the original Python implementation:
//! `~/.config/grafl/config.json` with fields like:
//! `{ "voice_provider": "piper", "log_level": "INFO" }`.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use dirs::config_dir;

use crate::model::{LogLevel, TTSBackend};

const APP_CONFIG_DIR_NAME: &str = "grafl";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
struct RawConfig {
    /// Voice provider name ("piper" or "polly").
    #[serde(default)]
    voice_provider: Option<String>,

    /// Log level string, kept for compatibility (unused for now).
    #[serde(default)]
    log_level: Option<String>,
}

fn config_path() -> Option<PathBuf> {
    let path = config_dir()?.join(APP_CONFIG_DIR_NAME).join(CONFIG_FILE_NAME);
    Some(path)
}

fn ensure_config_dir_exists(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn load_raw_config() -> Result<RawConfig, ConfigError> {
    let Some(path) = config_path() else {
        // No config directory available on this platform; treat as empty config.
        eprintln!("Config: no config_dir available, using defaults only");
        return Ok(RawConfig::default());
    };

    if !path.exists() {
        eprintln!("Config: file does not exist, using defaults ({path:?})");
        return Ok(RawConfig::default());
    }

    let data = fs::read_to_string(&path)?;
    let cfg = serde_json::from_str(&data)?;
    eprintln!("Config: loaded from {path:?}");
    Ok(cfg)
}

fn save_raw_config(mut cfg: RawConfig) -> Result<(), ConfigError> {
    let Some(path) = config_path() else {
        // Nothing we can do; silently ignore.
        eprintln!("Config: no config_dir available, skipping save");
        return Ok(());
    };

    ensure_config_dir_exists(&path)?;
    // Normalize by dropping unknown providers / empty strings if present.
    if cfg.voice_provider.as_deref().map_or(true, |s| s.is_empty()) {
        cfg.voice_provider = None;
    }

    if cfg.log_level.as_deref().map_or(true, |s| s.is_empty()) {
        cfg.log_level = None;
    }

    let data = serde_json::to_string_pretty(&cfg)?;
    fs::write(&path, data)?;
    eprintln!("Config: saved to {path:?}");
    Ok(())
}

fn backend_from_str(s: &str) -> Option<TTSBackend> {
    match s {
        "piper" => Some(TTSBackend::Piper),
        "polly" => Some(TTSBackend::AwsPolly),
        _ => None,
    }
}

fn backend_to_str(backend: TTSBackend) -> &'static str {
    match backend {
        TTSBackend::Piper => "piper",
        TTSBackend::AwsPolly => "polly",
    }
}

fn log_level_from_str(s: &str) -> Option<LogLevel> {
    match s.to_ascii_uppercase().as_str() {
        "ERROR" => Some(LogLevel::Error),
        "WARN" | "WARNING" => Some(LogLevel::Warn),
        "INFO" => Some(LogLevel::Info),
        "DEBUG" => Some(LogLevel::Debug),
        "TRACE" => Some(LogLevel::Trace),
        _ => None,
    }
}

fn log_level_to_str(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warn => "WARN",
        LogLevel::Info => "INFO",
        LogLevel::Debug => "DEBUG",
        LogLevel::Trace => "TRACE",
    }
}

/// Load the persisted voice provider, defaulting to Piper if not set or invalid.
pub fn load_voice_provider() -> TTSBackend {
    let backend = match load_raw_config() {
        Ok(cfg) => cfg
            .voice_provider
            .as_deref()
            .and_then(backend_from_str)
            .unwrap_or(TTSBackend::Piper),
        Err(err) => {
            eprintln!("Config: failed to load config, using default backend: {err:?}");
            TTSBackend::Piper
        }
    };
    eprintln!("Config: effective voice provider on load = {:?}", backend);
    backend
}

/// Load the persisted log level, defaulting to `Info` if not set or invalid.
pub fn load_log_level() -> LogLevel {
    match load_raw_config() {
        Ok(cfg) => cfg
            .log_level
            .as_deref()
            .and_then(log_level_from_str)
            .unwrap_or(LogLevel::Info),
        Err(err) => {
            eprintln!("Config: failed to load config, using default log level: {err:?}");
            LogLevel::Info
        }
    }
}

/// Persist the selected voice provider to disk.
///
/// Errors are logged to stderr and otherwise ignored.
pub fn save_voice_provider(backend: TTSBackend) {
    eprintln!("Config: saving voice provider = {:?}", backend);
    let mut cfg = match load_raw_config() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Config: failed to load existing config, starting fresh: {err:?}");
            RawConfig::default()
        }
    };

    cfg.voice_provider = Some(backend_to_str(backend).to_string());

    if let Err(err) = save_raw_config(cfg) {
        eprintln!("Failed to save config: {err:?}");
    }
}

/// Persist the selected log level to disk.
///
/// Errors are logged to stderr and otherwise ignored.
pub fn save_log_level(level: LogLevel) {
    eprintln!("Config: saving log level = {:?}", level);
    let mut cfg = match load_raw_config() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Config: failed to load existing config, starting fresh: {err:?}");
            RawConfig::default()
        }
    };

    cfg.log_level = Some(log_level_to_str(level).to_string());

    if let Err(err) = save_raw_config(cfg) {
        eprintln!("Failed to save config: {err:?}");
    }
}


