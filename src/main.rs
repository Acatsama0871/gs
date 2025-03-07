mod modules;
use clap::{Parser, Subcommand};
use colored::Colorize;
use modules::show;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "gs")]
#[command(version = "0.0.0")]
#[command(about = "A simple CLI tool for interacting with Google Scholar")]
struct CliParser {
    #[command(subcommand)]
    subcmd: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    #[command(name = "show")]
    #[command(about = "Show the citation info.")]
    Show {},
    #[command(name = "log")]
    #[command(about = "Log the current info as a checkpoint")]
    LogCheckpoint {},
}

fn main() -> ExitCode {
    let cli_args = CliParser::parse();

    match cli_args.subcmd {
        Some(Subcommands::Show {}) => {
            if let Err(e) = show::show_func() {
                eprintln!("{}", format!("{}", e).red());
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Some(Subcommands::LogCheckpoint {}) => {
            todo!()
        }
        None => {
            ExitCode::SUCCESS
        }
    }
}
