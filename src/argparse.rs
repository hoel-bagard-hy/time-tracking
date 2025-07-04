use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the timesheet.
    #[arg(
        short('l'),
        long,
        default_value=dirs::home_dir().unwrap().join(".local/share/time-tracking/timesheet.csv").into_os_string()
    )]
    pub timesheet_path: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Starting to work.
    Start {
        /// Task name
        task_name: String,
    },

    ///Switching to another task.
    Switch {
        /// Task name
        task_name: String,
    },

    /// Finished working.
    End,

    /// Print all the tasks present in the timesheet.
    TaskNames,

    /// Read the timesheet, and print how much time was spend on each task in a way that is easy to copy/paste into the excel sheet.
    Times {
        /// Date for which to retrieve the timesheet, using the %Y-%m-%d format. For example: "2015-09-05".
        target_date: String,
    },
}
