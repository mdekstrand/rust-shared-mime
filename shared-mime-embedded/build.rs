use std::env;
use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use shared_mime::runtime::parse_mime_package;

const SHARED_MIME_FILE: &str = "shared-mime-info/data/freedesktop.org.xml.in";

fn main() -> Result<()> {
    eprintln!("parsing {}", SHARED_MIME_FILE);
    let file = PathBuf::from(SHARED_MIME_FILE);
    let pkg = parse_mime_package(&file)?;
    let records = pkg.into_records();
    let out_dir = env::var("OUT_DIR")?;
    let out_fn = format!("{}/mimedata.bin", out_dir);
    let mut out = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&out_fn)?;
    postcard::to_io(&records, &mut out)?;
    println!("cargo:rerun-if-changed={}", SHARED_MIME_FILE);
    println!("cargo:rustc-env=EMBEDDED_MIME_PATH={}", out_fn);
    Ok(())
}
