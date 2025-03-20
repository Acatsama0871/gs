mod modules;
use clap::{Parser, Subcommand, builder::EnumValueParser};
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
    Show {
        #[arg(
            short = 'p',
            long = "pages",
            default_value = "1",
            help = "Control the number of articles to show on each Google Scholar page, setting to -1 will show all pages, setting to 0 will suppress the output."
        )]
        pages: i16,

        #[arg(
            short = 's',
            long = "suppress-author",
            default_value = "false",
            help = "Whether to suppress the author's info"
        )]
        suppress_author: bool,

        #[arg(
            short = 'f',
            long = "find-author",
            help = "Find the author by citations id or name"
        )]
        find_author: Option<String>,

        #[arg(
                short = 'o',
                long = "output-format",
                default_value = "cli-table",
                help = "Output format for the citation data",
                value_parser =EnumValueParser::<show::OutputFormat>::new()
            )]
        output_format: show::OutputFormat,
    },
    #[command(name = "log")]
    #[command(about = "Log the current info as a checkpoint")]
    LogCheckpoint {},
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli_args = CliParser::parse();

    match cli_args.subcmd {
        Some(Subcommands::Show {
            pages,
            suppress_author,
            find_author,
            output_format
        }) => {
            if let Err(e) = show::show_func(pages, suppress_author, find_author, output_format).await {
                eprintln!("{}", format!("{}", e).red());
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Some(Subcommands::LogCheckpoint {}) => {
            todo!()
        }
        None => ExitCode::SUCCESS,
    }
}
