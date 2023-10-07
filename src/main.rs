use chrono::prelude::*;
use std::{fs, thread};
use std::collections::HashSet;
use std::str::FromStr;
use chrono::Duration;
use clap::Parser;
use console::Term;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long)]
    /// Optional path to file containing holidays, each in its own line in %Y-%m-%d format e.g.: 2023-10-09
    holiday_path: Option<String>,

    /// Optionally calculate from a given time, in %Y-%m-%dT%H:%M:%S%.f format, e.g.: 2023-10-09T12:00:00.0
    #[arg(long)]
    now: Option<NaiveDateTime>,

    /// The last day at work in %Y-%m-%d format e.g.: 2023-11-17
    last_workday: NaiveDate,
    /// The time when you end work in %H:%M format, e.g: 16:00. An 8 hour shift is assumed.
    end_of_worktime: NaiveTime
}

fn parse_holidays(args: &Args) -> HashSet<NaiveDate> {
    let mut holidays = HashSet::new();
    if let Some(holiday_path) = &args.holiday_path {
        let holiday_file_string = fs::read_to_string(holiday_path).expect("Unable to read holiday file");
        for holiday_str in holiday_file_string.split('\n') {
            if let Ok(holiday) = NaiveDate::from_str(holiday_str) {
                holidays.insert(holiday);
            }
        }
    }
    holidays
}

fn calculate_diff_for_today(now: &NaiveDateTime, args: &Args) -> f64{
    if now.weekday().number_from_monday() <= 5 {
        let seconds_remaining_today = (args.end_of_worktime - now.time()).num_seconds();
        let today_remaining = seconds_remaining_today as f64 / (8 * 60 * 60) as f64;
        today_remaining.max(0.0)
    }
    else {
        0.0
    }
}

fn main() {
    let args = Args::parse();
    let holidays = parse_holidays(&args);

    let term = Term::stdout();
    loop {
        let mut diff = 0.0;
        let mut now = Local::now().naive_local();
        if let Some(new_now) = args.now {
            now = new_now;
        }

        diff += calculate_diff_for_today(&now, &args);

        let mut date_to_check = now.date() + Duration::days(1);
        while date_to_check <= args.last_workday {
            if date_to_check.weekday().number_from_monday() <= 5 && !holidays.contains(&date_to_check){
                diff += 1.0;
            }
            date_to_check += Duration::days(1);
        }

        term.clear_last_lines(1).unwrap();
        println!("Workdays remaining: {}", diff);
        if let Some(_) = args.now {
            break;
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
}