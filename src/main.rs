mod asy;
mod color;
mod group;
mod path;
mod pen;
mod svg2asy;

use std::path::PathBuf;
use std::{env, fs};

use anyhow::{Context, Result};
use clap::Parser;

use crate::svg2asy::svg2asy;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    file: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let options = Options::parse();

    let input = fs::canonicalize(options.file).context("Invalid input file")?;
    let output = options.output.map_or_else(
        || -> Result<_> {
            let mut output = env::current_dir().context("Failed to get current directory")?;
            output.push(input.file_stem().context("Invalid input file")?);
            output.set_extension("asy");
            Ok(output)
        },
        Ok,
    )?;

    svg2asy(input, output)
}
