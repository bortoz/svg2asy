mod asy;
mod color;
mod group;
mod path;
mod pen;
mod svg2asy;

use std::path::PathBuf;
use std::{env, fs};

use anyhow::{ensure, Context, Result};
use clap::{Args, Parser};

use crate::svg2asy::svg2asy;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    file: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    #[command(flatten)]
    asy_options: AsyOptions,
}

#[derive(Args)]
pub struct AsyOptions {
    #[arg(
        short,
        long,
        default_value = "6",
        help = "Number of decimal places for floating point numbers"
    )]
    pub precision: u32,
}

fn main() -> Result<()> {
    let options = Options::parse();

    let input = fs::canonicalize(options.file).context("Invalid input file")?;
    let output = if let Some(output) = options.output {
        if let Some(extension) = output.extension() {
            ensure!(
                extension == "asy",
                "Output file must have \".asy\" extension"
            );
            output
        } else {
            output.with_extension("asy")
        }
    } else {
        let mut output = env::current_dir().context("Failed to get current directory")?;
        output.push(input.file_stem().context("Invalid input file")?);
        output.set_extension("asy");
        output
    };

    svg2asy(input, output, &options.asy_options)
}
