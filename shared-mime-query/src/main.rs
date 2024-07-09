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
use shared_mime::MimeDB;
use stderrlog::StdErrLog;

use shared_mime::load_mime_db as load_xdg_mime_db;
use shared_mime::runtime::mimeinfo::load_xdg_mime_info;
use shared_mime::runtime::parse_mime_package;
use shared_mime::runtime::xdg_mime_search_dirs;
#[cfg(feature = "embedded")]
use shared_mime_embedded::{embedded_mime_db, load_mime_db as load_joint_mime_db};

/// Tools to query MIME data and debug the MIME engine.
#[derive(Parser)]
#[command()]
pub struct CLI {
    #[command(flatten)]
    action: MIMEActions,

    /// MIME data pacakge file(s).
    #[arg(short = 'p', long = "package")]
    pkg_files: Vec<PathBuf>,

    /// Enable verbose diagnostic logging.
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,

    /// Suppress informational outputs.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Specify output file for compilation.
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Output JSON where appropriate.
    #[arg(long = "json")]
    json: bool,

    /// Only use the embeded MIME db.
    #[arg(long = "no-runtime")]
    no_runtime: bool,

    /// Only use the runtime MIME db (if available).
    #[arg(long = "no-embedded")]
    no_embedded: bool,
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
    fn load_db(&self) -> Result<MimeDB> {
        if self.no_embedded && self.no_runtime {
            warn!("no XDG source specified");
            return Ok(MimeDB::new());
        }

        #[cfg(feature = "embedded")]
        if self.no_runtime {
            info!("loading embedded MIME database");
            return Ok(embedded_mime_db());
        } else if !self.no_embedded {
            info!("loading joint MIME database");
            return Ok(load_joint_mime_db()?);
        }

        info!("loading runtime MIME database");
        return Ok(load_xdg_mime_db()?);
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
        let db = self.load_db()?;
        if let Some(name) = path.file_name() {
            info!("looking up type for {}", path.display());
            let ans = db.match_filename(name);
            let all = ans.all_types();
            if let Some(mt) = ans.best() {
                println!("{}: {}", path.display(), mt);
                if all.len() > 1 {
                    info!("file has {} other types", all.len() - 1);
                }
            } else if ans.is_unknown() {
                error!("{}: unknown type", path.display());
            } else if ans.is_ambiguous() {
                warn!("{}: ambiguous type", path.display());
                for mt in all {
                    println!("{}: {}", path.display(), mt);
                }
            }
            Ok(())
        } else {
            error!("{}: path has no filename", path.display());
            Err(anyhow!("invalid path"))
        }
    }
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    StdErrLog::new()
        .verbosity(if cli.quiet {
            1
        } else {
            cli.verbose as usize + 2
        })
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
