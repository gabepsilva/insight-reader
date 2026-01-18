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
        Some(Commands::Config { action: ConfigAction::Show }) => show_config(),
        Some(Commands::Logs { action: LogsAction::Show { lines } }) => show_logs(lines),
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
            eprintln!(
                "Error reading log directory {}: {}",
                log_dir.display(),
                e
            );
            std::process::exit(1);
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
    let file = fs::File::open(&latest_log).unwrap_or_else(|e| {
        eprintln!("Error opening log file: {}", e);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    let all_lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap_or_else(|e| {
            eprintln!("Error reading log file: {}", e);
            std::process::exit(1);
        });

    let start = all_lines.len().saturating_sub(lines);
    let lines_to_show = &all_lines[start..];

    // Print log lines first
    for line in lines_to_show {
        println!("{}", line);
    }

    // Print header information last
    println!();
    println!("Log file: {}", latest_log.display());
    println!("Showing last {} line(s):", lines_to_show.len());
}
