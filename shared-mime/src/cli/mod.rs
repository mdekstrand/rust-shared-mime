use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

use anyhow::Result;
use clap::{Args, Parser};
use log::*;
use serde_json::to_string;
use stderrlog::StdErrLog;

use crate::runtime::parse_mime_package;
use crate::runtime::xdg_mime_search_dirs;

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

    /// Compile to JSON.
    #[arg(long = "json", requires = "compile")]
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
}

impl CLI {
    pub fn run(&self) -> Result<()> {
        StdErrLog::new()
            .verbosity(self.verbose as usize + 2)
            .init()
            .expect("log setup error");

        info!("CLI launching");
        if self.action.list_dirs {
            self.list_dirs()
        } else if self.action.compile {
            self.compile()
        } else {
            error!("no specified action");
            exit(2)
        }
    }

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
            let mut out: Box<dyn Write> = if let Some(op) = &self.output {
                Box::new(
                    File::options()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(op)?,
                )
            } else {
                Box::new(std::io::stderr())
            };
            for rec in records {
                writeln!(out, "{}", to_string(&rec)?)?;
            }
        } else {
            error!("cannot output");
        }
        Ok(())
    }
}
