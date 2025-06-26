use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ReportMode {
    /// Print all the tasks present in the timesheet.
    TaskNames,
    /// Print how much time was spend on each task
    Times,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the timesheet.
    #[arg(
        short('l'),
        long,
        default_value("~/.local/share/time-tracking/timesheet.csv")
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

    /// Report information entered in the timesheet in a way that is easy to copy/paste into the excel sheet.
    Report {
        /// Action to perform.
        mode: ReportMode,
        /// Date for which to retrieve the timesheet, using the %Y-%m-%d format. For example: "2015-09-05".
        target_date: String,
    },
}
