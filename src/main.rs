use cirro::cli_args::{CliArgs, Commands};
use cirro::{authenticate, run_tui};
use clap::Parser;

fn main() {
    let cli_args = CliArgs::parse();
    match cli_args.command {
        Some(Commands::Authenticate) => authenticate(),
        None => run_tui(),
    }
}
