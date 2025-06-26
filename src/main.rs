#![allow(clippy::module_name_repetitions)]
use anyhow::Result;
use clap::Parser;

mod argparse;
use crate::argparse::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{:?}", cli);

    Ok(())
}
