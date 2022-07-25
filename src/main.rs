use clap::{Parser, Subcommand};
use std::env;

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
  ListEmptyFiles {
    #[clap(short, long, value_parser, default_value = "")]
    directory: String,
  },
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
    Commands::ListEmptyFiles { directory } => {
      let cwd = match env::current_dir() {
          Ok(path) => String::from(path.to_str().unwrap()),
          Err(e) => {
            println!("{}", e);
            return
          }
        };

      commands::list_empty_files(
          if directory == "" { cwd } else { directory }
      );
    }
    Commands::ListReferencelessFiles => {
      commands::list_referenceless_files();
    }
  }
}
