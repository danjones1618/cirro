use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Cirro unifies development dashboards into a single TUI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Use a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Setup API tokens for configured sources.
    Authenticate,
}
