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
