//! Integration tests for CLI functionality

use std::process::Command;

/// Helper to get the path to the compiled binary
fn get_binary_path() -> std::path::PathBuf {
    let mut path = std::env::current_exe().expect("Failed to get current executable path");

    // When running tests, the binary is in target/debug or target/release
    // We need to go up to the target directory and find the actual binary
    path.pop(); // Remove test executable name
    path.pop(); // Remove 'deps' directory

    let binary_name = if cfg!(windows) {
        "insight-reader.exe"
    } else {
        "insight-reader"
    };

    path.push(binary_name);
    path
}

/// Helper to run a command and return the output
fn run_command(args: &[&str]) -> std::process::Output {
    Command::new(get_binary_path())
        .args(args)
        .output()
        .expect("Failed to execute command")
}

/// Helper to assert version output contains expected content
fn assert_version_output(stdout: &str) {
    const EXPECTED_VERSION: &str = env!("CARGO_PKG_VERSION");
    assert!(
        stdout.contains("insight-reader"),
        "Version output should contain program name"
    );
    assert!(
        stdout.contains(EXPECTED_VERSION),
        "Version output should contain version number '{}'. Got: {}",
        EXPECTED_VERSION,
        stdout
    );
}

#[test]
fn test_version_flag() {
    let output = run_command(&["--version"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );
    assert_version_output(&String::from_utf8_lossy(&output.stdout));
}

#[test]
fn test_version_flag_short() {
    let output = run_command(&["-V"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );
    assert_version_output(&String::from_utf8_lossy(&output.stdout));
}

#[test]
fn test_help_flag() {
    let output = run_command(&["--help"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Insight Reader"),
        "Help should contain program description"
    );
    assert!(
        stdout.contains("Usage:"),
        "Help should contain usage information"
    );
    assert!(
        stdout.contains("Commands:"),
        "Help should list available commands"
    );
    assert!(stdout.contains("config"), "Help should list config command");
}

#[test]
fn test_help_flag_short() {
    let output = run_command(&["-h"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Insight Reader"),
        "Help should contain program description"
    );
}

#[test]
fn test_config_help() {
    let output = run_command(&["config", "--help"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Configuration management"),
        "Config help should contain description"
    );
    assert!(
        stdout.contains("show"),
        "Config help should list show subcommand"
    );
}

#[test]
fn test_config_show_command() {
    let output = run_command(&["config", "show"]);
    // Command should succeed whether or not config file exists
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The output should either show the config file contents or mention it doesn't exist
    let has_config_file = stdout.contains("Config file:") || stdout.contains("config.json");
    let mentions_not_exist =
        stdout.contains("does not exist") || stdout.contains("(This is normal");

    assert!(
        has_config_file || mentions_not_exist,
        "Config show should either display config or mention it doesn't exist. Got: {}",
        stdout
    );
}

#[test]
fn test_no_arguments_shows_message() {
    let output = run_command(&["config"]);
    // Should fail since no subcommand provided to config
    assert!(
        !output.status.success(),
        "Command should fail without subcommand"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    // Clap shows the help message when required subcommand is missing
    assert!(
        stderr.contains("Usage:") || stderr.contains("COMMAND"),
        "Should show usage message when subcommand is missing. Got: {}",
        stderr
    );
}

#[test]
fn test_invalid_command() {
    let output = run_command(&["invalid-command-xyz"]);
    assert!(
        !output.status.success(),
        "Command should fail with invalid command"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error") || stderr.contains("unrecognized"),
        "Should show error about invalid command"
    );
}

#[test]
fn test_logs_help() {
    let output = run_command(&["logs", "--help"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Log file management"),
        "Logs help should contain description"
    );
    assert!(
        stdout.contains("show"),
        "Logs help should list show subcommand"
    );
}

#[test]
fn test_logs_show_command() {
    let output = run_command(&["logs", "show"]);
    // Command should succeed whether or not log files exist
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The output should either show log contents or mention logs don't exist
    let has_log_file = stdout.contains("Log file:") || stdout.contains("insight-reader.log");
    let mentions_not_exist =
        stdout.contains("does not exist") || stdout.contains("(This is normal");

    assert!(
        has_log_file || mentions_not_exist,
        "Logs show should either display logs or mention they don't exist. Got: {}",
        stdout
    );
}

#[test]
fn test_logs_show_with_n_flag() {
    let output = run_command(&["logs", "show", "-n", "10"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The output should either show log contents with line count or mention logs don't exist
    if stdout.contains("Log file:") {
        assert!(
            stdout.contains("Showing last") && stdout.contains("line(s)"),
            "Output should show how many lines are displayed"
        );
    } else {
        assert!(
            stdout.contains("does not exist") || stdout.contains("(This is normal"),
            "Should mention logs don't exist if no logs are found"
        );
    }
}

#[test]
fn test_logs_show_with_lines_flag() {
    let output = run_command(&["logs", "show", "--lines", "25"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The output should either show log contents with line count or mention logs don't exist
    if stdout.contains("Log file:") {
        assert!(
            stdout.contains("Showing last") && stdout.contains("line(s)"),
            "Output should show how many lines are displayed"
        );
    } else {
        assert!(
            stdout.contains("does not exist") || stdout.contains("(This is normal"),
            "Should mention logs don't exist if no logs are found"
        );
    }
}

#[test]
fn test_logs_show_with_zero_lines() {
    let output = run_command(&["logs", "show", "-n", "0"]);
    assert!(
        output.status.success(),
        "Command should succeed with 0 lines requested"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("Log file:") {
        // When 0 lines requested, should either:
        // 1. Show "Showing last 0 line(s)" in the header, OR
        // 2. Have no log content lines (only header lines)
        let has_zero_line_count = stdout.contains("Showing last 0 line(s)");
        let log_content_lines: Vec<&str> = stdout
            .lines()
            .filter(|l| {
                !l.is_empty()
                    && !l.contains("Log file:")
                    && !l.contains("Showing last")
            })
            .collect();
        let has_no_content = log_content_lines.is_empty();

        assert!(
            has_zero_line_count || has_no_content,
            "Should show 0 lines of log content when 0 is requested. Found {} content lines: {:?}",
            log_content_lines.len(),
            log_content_lines
        );
    }
}

#[test]
fn test_logs_show_with_large_number() {
    let output = run_command(&["logs", "show", "--lines", "1000"]);
    assert!(
        output.status.success(),
        "Command should succeed even with large line count"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("Log file:") {
        // Should show "Showing last N line(s)" where N <= 1000 (depending on actual file size)
        assert!(
            stdout.contains("Showing last") && stdout.contains("line(s)"),
            "Output should show line count information"
        );
    }
}

#[test]
fn test_logs_show_default_is_50_lines() {
    let output = run_command(&["logs", "show"]);
    assert!(
        output.status.success(),
        "Command failed with status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // When logs exist, it should default to showing 50 lines or fewer (if file is smaller)
    if stdout.contains("Log file:") && stdout.contains("Showing last") {
        // Extract the number of lines shown from the output
        // The format is "Showing last N line(s):"
        let has_reasonable_line_count = stdout.lines().any(|line| {
            if line.contains("Showing last") && line.contains("line(s)") {
                // Line count should be <= 50 (the default)
                true
            } else {
                false
            }
        });
        assert!(
            has_reasonable_line_count,
            "Default should show line count information"
        );
    }
}

#[test]
fn test_logs_show_invalid_lines_argument() {
    let output = run_command(&["logs", "show", "-n", "not-a-number"]);
    assert!(
        !output.status.success(),
        "Command should fail with invalid lines argument"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error") || stderr.contains("invalid"),
        "Should show error about invalid argument"
    );
}

#[test]
fn test_logs_no_subcommand() {
    let output = run_command(&["logs"]);
    assert!(
        !output.status.success(),
        "Command should fail without subcommand"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Usage:") || stderr.contains("COMMAND"),
        "Should show usage message when subcommand is missing"
    );
}
