use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};

use anyhow::{Result, bail};
use chrono::{NaiveDate, NaiveDateTime, TimeDelta};

use crate::argparse::ReportMode;

/// Read a CSV written by the log command, and print out its content in a way that is easily copy/pastable into an excel sheet.
#[allow(clippy::expect_used)]
pub fn process_csv(
    log_file_path: &PathBuf,
    target_date: &str,
) -> Result<HashMap<String, TimeDelta>> {
    // Map each task to the total amount of time worked.
    let mut worked_times = HashMap::new();
    let target_date = NaiveDate::parse_from_str(target_date, "%Y-%m-%d")?;

    for line in fs::read_to_string(log_file_path)?.lines() {
        let [task, start_time_str, end_time_str] = line.split(',').collect::<Vec<&str>>()[..]
        else {
            bail!("Found malformed line: {}", line)
        };

        let start_time = NaiveDateTime::parse_from_str(start_time_str, "%Y-%m-%d %H:%M:%S")?;
        let end_time = NaiveDateTime::parse_from_str(end_time_str, "%Y-%m-%d %H:%M:%S")?;

        if start_time.date() != target_date {
            continue;
        }

        let duration = end_time - start_time;
        if duration.num_seconds() < 0 {
            bail!("Found start time later than end time: {}", line);
        }
        if duration.num_hours() > 24 {
            bail!("Found abnormally work duration: {}", line);
        }

        worked_times
            .entry(task.to_owned())
            .and_modify(|time| *time += duration)
            .or_insert(duration);
    }

    Ok(worked_times)
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::expect_used
)]
pub fn print_report(mode: ReportMode, worked_times: &HashMap<String, TimeDelta>) {
    let mut tasks: Vec<&String> = worked_times.keys().collect();
    tasks.sort();
    match mode {
        ReportMode::TaskNames => {
            for task in tasks {
                println!("{task}");
            }
        }
        ReportMode::Times => {
            for task in tasks {
                println!(
                    "{}h{:02}min",
                    worked_times
                        .get(task)
                        .unwrap_or(&TimeDelta::zero())
                        .num_hours(),
                    worked_times
                        .get(task)
                        .unwrap_or(&TimeDelta::zero())
                        .num_minutes()
                        % 60
                );
            }
        }
    }
}
