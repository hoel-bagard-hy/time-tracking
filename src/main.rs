#![allow(clippy::module_name_repetitions)]
use anyhow::Result;
use clap::Parser;
use log::{log_end, log_start, log_switch};

mod argparse;
mod log;
use crate::argparse::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{:?}", cli);

    match &cli.command {
        Commands::Start { task_name } => {
            log_start(task_name, &cli.timesheet_path)?;
        }
        Commands::Switch { task_name } => {
            log_switch(task_name, &cli.timesheet_path)?;
        }
        Commands::End => {
            log_end(&cli.timesheet_path)?;
        }
        Commands::Report { mode, target_date } => {
            println!("Report mode: {:?}, target date: {target_date}", mode);
        }
    }
    Ok(())
}
