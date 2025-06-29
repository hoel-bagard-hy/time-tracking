#![allow(clippy::module_name_repetitions)]
use anyhow::Result;
use clap::Parser;
use log::{log_end, log_start, log_switch};
use report::{print_report, process_csv};

mod argparse;
mod log;
mod report;
use crate::argparse::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

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
            let worked_times = process_csv(&cli.timesheet_path, target_date)?;
            print_report(*mode, &worked_times);
        }
    }
    Ok(())
}
