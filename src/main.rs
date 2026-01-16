//! Entry point and window configuration

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod config;
mod flags;
mod logging;
mod model;
mod providers;
mod styles;
mod system;
mod update;
mod ui;
mod view;
mod voices;

use iced::daemon;
use tracing::info;

fn main() -> iced::Result {
    // Check for existing instance before doing anything else
    let _instance_guard = match crate::system::try_lock() {
        Ok(guard) => guard,
        Err(crate::system::SingleInstanceError::LockFailed) => {
            eprintln!("Error: Another instance of Insight Reader is already running.");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error: Failed to check for existing instance: {e}");
            std::process::exit(1);
        }
    };

    // Initialize logging first (before anything else)
    let log_config = logging::LoggingConfig {
        verbosity: config::load_log_level(),
        log_to_stderr: true,
        log_to_file: true,
        log_dir: None, // Use default: ~/.local/share/insight-reader/logs
    };

    if let Err(e) = logging::init_logging(&log_config) {
        eprintln!("Failed to initialize logging: {e}");
        // Continue anyway - app can run without logging
    }

    info!("Insight Reader starting up");

    // Use daemon for multi-window support (view receives window::Id)
    // Note: Text selection is now fetched asynchronously after UI appears for blazing fast startup
    daemon(crate::app::new, crate::app::update, crate::app::view)
        .title(crate::app::title)
        .subscription(crate::app::subscription)
        .run()
}
