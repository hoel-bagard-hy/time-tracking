#![allow(clippy::module_name_repetitions)]
use anyhow::Result;
use clap::Parser;
use log::log_start;

mod argparse;
mod log;
use crate::argparse::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{:?}", cli);

    match &cli.command {
        Commands::Start { task_name } => {
            println!("Start mode");
            log_start(task_name, &cli.timesheet_path)?;
        }
        Commands::Switch { task_name } => {
            println!("Log mode: {task_name}")
        }
        Commands::End => {
            println!("Log mode")
        }
        Commands::Report { mode, target_date } => {
            println!("Report mode: {:?}, target date: {target_date}", mode)
        }
    }
    Ok(())
}
