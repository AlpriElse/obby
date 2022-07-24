use clap::{Parser, Subcommand};

mod commands;

/// CLI for wrangling Obsidian markdown files
#[derive(Parser)]
#[clap(name = "Obby")]
struct Cli {
    /// Name of the program to run
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    ConvertDates,
    ListDanglingLinks,
    ListEmptyFiles,
    ListReferencelessFiles
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::ConvertDates => {
            commands::convert_dates();
        }
        Commands::ListDanglingLinks => {
            commands::list_dangling_links();
        }
        Commands::ListEmptyFiles => {
            commands::list_empty_files();
        }
        Commands::ListReferencelessFiles => {
            commands::list_referenceless_files();
        }
    }
}
