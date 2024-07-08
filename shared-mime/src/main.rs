use clap::Parser;

use anyhow::Result;
use shared_mime::cli::CLI;

fn main() -> Result<()> {
    let cli = CLI::parse();
    cli.run()
}
