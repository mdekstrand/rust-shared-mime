use std::fs::File;
use std::io;
use std::io::IsTerminal;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

use anyhow::anyhow;
use anyhow::Result;
use clap::{Args, Parser};
use log::*;
use serde_json::{to_string, to_writer_pretty};
use stderrlog::StdErrLog;

use shared_mime::runtime::mimeinfo::load_xdg_mime_info;
use shared_mime::runtime::parse_mime_package;
use shared_mime::runtime::xdg_mime_search_dirs;

/// Tools to query MIME data and debug the MIME engine.
#[derive(Parser)]
#[command()]
pub struct CLI {
    #[command(flatten)]
    action: MIMEActions,

    /// MIME data pacakge file(s).
    #[arg(short = 'p', long = "package")]
    pkg_files: Vec<PathBuf>,

    /// Enable verbose diagnostic logging
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,

    /// Specify output file for compilation.
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Output JSON where appropriate.
    #[arg(long = "json")]
    json: bool,
}

#[derive(Args)]
#[group(multiple = false, required = true)]
pub struct MIMEActions {
    /// List MIME search directories.
    #[arg(long = "list-dirs")]
    list_dirs: bool,

    /// Compile a MIME database.
    #[arg(long = "compile")]
    compile: bool,

    /// Dump the MIME information.
    #[arg(long = "dump")]
    dump: bool,

    /// Query the type of a file.
    #[arg(short = 'T', long = "type-of")]
    type_of: Option<PathBuf>,
}

impl CLI {
    fn list_dirs(&self) -> Result<()> {
        for dir in xdg_mime_search_dirs() {
            println!("{}", dir.display());
        }
        Ok(())
    }

    fn compile(&self) -> Result<()> {
        if self.pkg_files.len() != 1 {
            error!("--compile must have exactly one package");
            exit(2)
        }
        let file = &self.pkg_files[0];
        let pkg = parse_mime_package(file)?;
        let records = pkg.into_records();
        if self.json {
            info!("compiling to JSON");
            let mut out = self.open_text_output()?;
            for rec in records {
                writeln!(out, "{}", to_string(&rec)?)?;
            }
        } else {
            info!("compiling to compressed binary");
            let mut out = self.open_bin_output()?;
            postcard::to_io(&records, &mut out)?;
        }
        Ok(())
    }

    fn dump(&self) -> Result<()> {
        info!("loading XDG mime info");
        let db = load_xdg_mime_info()?;
        let out = self.open_text_output()?;
        if self.json {
            to_writer_pretty(out, &db.directories)?;
        } else {
            for dir in db.directories {
                println!(
                    "directory {} ({} packages):",
                    dir.path.display(),
                    dir.packages.len()
                );
                for pkg in dir.packages {
                    println!("  package {} ({} types):", pkg.filename, pkg.types.len());
                    for t in pkg.types {
                        println!("  - {:?}", t)
                    }
                }
            }
        }
        Ok(())
    }

    fn open_text_output(&self) -> Result<Box<dyn Write>> {
        let out: Box<dyn Write> = if let Some(op) = &self.output {
            Box::new(
                File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(op)?,
            )
        } else {
            Box::new(std::io::stdout())
        };
        Ok(out)
    }

    fn open_bin_output(&self) -> Result<Box<dyn Write>> {
        let out: Box<dyn Write> = if let Some(op) = &self.output {
            Box::new(
                File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(op)?,
            )
        } else if io::stdout().is_terminal() {
            error!("standard output is a terminal, refusing to write binary");
            return Err(anyhow!("terminals do not get binary output"));
        } else {
            Box::new(std::io::stdout())
        };
        Ok(out)
    }

    fn type_of(&self, path: &Path) -> Result<()> {
        todo!()
    }
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    StdErrLog::new()
        .verbosity(cli.verbose as usize + 2)
        .init()
        .expect("log setup error");

    info!("CLI launching");
    if cli.action.list_dirs {
        cli.list_dirs()
    } else if cli.action.compile {
        cli.compile()
    } else if cli.action.dump {
        cli.dump()
    } else if let Some(path) = &cli.action.type_of {
        cli.type_of(path)
    } else {
        error!("no specified action");
        exit(2)
    }
}
