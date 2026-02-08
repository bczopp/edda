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
fn test_parse_prompt_command() {
    let cli = parse(&["prompt", "What is 2+2?"]);
    match &cli.command {
        Some(Commands::Prompt { prompt }) => assert_eq!(prompt, "What is 2+2?"),
        _ => panic!("expected Prompt command"),
    }
}

#[test]
fn test_parse_models_command() {
    let cli = parse(&["models"]);
    match &cli.command {
        Some(Commands::Models) => {}
        _ => panic!("expected Models command"),
    }
}

#[test]
fn test_parse_retrieve_command() {
    let cli = parse(&["retrieve", "search term"]);
    match &cli.command {
        Some(Commands::Retrieve { query }) => assert_eq!(query, "search term"),
        _ => panic!("expected Retrieve command"),
    }
}

#[test]
fn test_parse_no_subcommand() {
    let cli = parse(&[]);
    assert!(cli.command.is_none());
}
