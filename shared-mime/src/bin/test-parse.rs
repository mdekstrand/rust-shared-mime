use std::path::PathBuf;

use clap::Parser;
use shared_mime_parser::parse::parse_mime_package;

#[derive(Parser)]
#[command()]
struct CLI {
    /// Database path to search.
    path: PathBuf,
}

fn main() {
    let cli = CLI::parse();
    match parse_mime_package(&cli.path) {
        Ok(info) => {
            for mt in info.types {
                println!("{:#?}", mt);
            }
        }
        Err(e) => {
            eprintln!("parse error: {:?}", e);
        }
    }
}
