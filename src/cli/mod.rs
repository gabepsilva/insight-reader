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
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show the current configuration
    Show,
}

/// Initialize logging for CLI mode (stderr only, respects RUST_LOG)
pub fn init_logging() {
    use crate::config;
    use crate::logging;

    let log_config = logging::LoggingConfig {
        verbosity: config::load_log_level(),
        log_to_stderr: true,
        log_to_file: false, // CLI doesn't need file logging
        log_dir: None,
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
    std::env::args().skip(1).next().is_some()
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
        Some(Commands::Config { action }) => match action {
            ConfigAction::Show => show_config(),
        },
    }
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
    let metadata = fs::metadata(&config_path).unwrap_or_else(|e| {
        eprintln!("Error reading config file metadata: {}", e);
        std::process::exit(1);
    });

    if metadata.len() > MAX_CONFIG_SIZE {
        eprintln!(
            "Error: Config file is too large ({} bytes, max {} bytes)",
            metadata.len(),
            MAX_CONFIG_SIZE
        );
        std::process::exit(1);
    }

    let contents = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        eprintln!("Error reading config file: {}", e);
        std::process::exit(1);
    });

    println!("Config file: {}", config_path.display());
    println!();
    println!("{}", contents);
}
