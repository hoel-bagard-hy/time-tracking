use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use anyhow::{Result, bail};
use chrono::Local;

/// Log the task name along with the current time stamp to the given file.
pub fn log_start(task_name: &str, timesheet_path: &PathBuf) -> Result<()> {
    // Get current time.
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = if timesheet_path.try_exists()? {
        // Check if the last line has both start and end times.
        let file = File::open(timesheet_path)?;
        let last_line = BufReader::new(file).lines().last().transpose()?;

        if let Some(line) = last_line {
            if line.split(',').count() != 3 {
                bail!("Last line was malformed: {}", line);
            }
        }

        // Open the file in append mode.
        OpenOptions::new().append(true).open(timesheet_path)?
    } else {
        println!("Log file does not exist, should it be created ? (Y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("y") || input.trim().is_empty() {
            File::create(timesheet_path)?
        } else {
            println!("File not created, exiting.");
            return Ok(());
        }
    };

    // Write current time to file.
    write!(file, "{task_name},{current_time}")?;

    Ok(())
}
