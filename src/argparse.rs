use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Subcommand, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    /// Starting to work.
    Start {
        /// Task name
        task_name: String,
    },
    ///Switching to another task
    Switch {
        /// Task name
        task_name: String,
    },
    /// Finished working.
    End,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Print all the tasks present in the timesheet.
    TaskNames,
    /// Print how much time was spend on each task
    Times,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the timesheet.
    #[arg(
        short('l'),
        long,
        default_value("~/.local/share/time-tracking/timesheet.csv")
    )]
    timesheet_path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Log worked hours.
    Log {
        #[command(subcommand)]
        action: Action,
    },

    /// Report information entered in the timesheet in a way that is easy to copy/paste into the excel sheet.
    Report {
        /// Action to perform.
        mode: Mode,
        /// Date for which to retrieve the timesheet, using the %Y-%m-%d format. For example: "2015-09-05".
        /// Transform to date using `NaiveDate::parse_from_str`
        target_date: String,
    },
}
