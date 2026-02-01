//! CLI parser tests (Phase 3): Command-Parsing, Argument-Validation.

use clap::Parser;
use ragnarok::cli::{Cli, Commands};

fn parse(args: &[&str]) -> Cli {
    let mut a = vec!["ragnarok"];
    a.extend(args);
    Cli::parse_from(a)
}

#[test]
fn test_parse_chat_command() {
    let cli = parse(&["chat", "hello world"]);
    match &cli.command {
        Some(Commands::Chat { message }) => assert_eq!(message, "hello world"),
        _ => panic!("expected Chat command"),
    }
}

#[test]
fn test_parse_action_command() {
    let cli = parse(&["action", "run_script"]);
    match &cli.command {
        Some(Commands::Action { action }) => assert_eq!(action, "run_script"),
        _ => panic!("expected Action command"),
    }
}

#[test]
fn test_parse_status_command() {
    let cli = parse(&["status"]);
    match &cli.command {
        Some(Commands::Status) => {}
        _ => panic!("expected Status command"),
    }
}

#[test]
fn test_parse_settings_command() {
    let cli = parse(&["settings"]);
    match &cli.command {
        Some(Commands::Settings) => {}
        _ => panic!("expected Settings command"),
    }
}

#[test]
fn test_parse_no_subcommand() {
    let cli = parse(&[]);
    assert!(cli.command.is_none());
}
