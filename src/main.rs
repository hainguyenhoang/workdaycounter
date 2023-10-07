use chrono::prelude::*;
use std::{fs, thread};
use std::collections::HashSet;
use std::str::FromStr;
use clap::Parser;
use console::Term;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long)]
    holiday_path: Option<String>,

    #[arg(long)]
    now: Option<NaiveDateTime>,

    last_workday: NaiveDate,
    end_of_worktime: NaiveTime
}

fn main() {
    let args = Args::parse();

    let mut holidays = HashSet::new();
    if let Some(holiday_path) = args.holiday_path {
        let holiday_file_string = fs::read_to_string(holiday_path).expect("Unable to read holiday file");
        for holiday_str in holiday_file_string.split('\n') {
            if let Ok(holiday) = NaiveDate::from_str(holiday_str) {
                holidays.insert(holiday);
            }
        }
    }

    let term = Term::stdout();
    loop {
        let mut diff = 0.0;
        let mut now = Local::now().naive_local();
        if let Some(new_now) = args.now {
            now = new_now;
        }

        if now.weekday().number_from_monday() <= 5 {
            let seconds_remaining_today = (args.end_of_worktime - now.time()).num_seconds();
            let today_remaining = seconds_remaining_today as f64 / (24 * 60 * 60) as f64;
            if today_remaining > 0.0 {
                diff += today_remaining;
            }
        }

        let mut date_to_check = now.date();
        while date_to_check < args.last_workday {
            if date_to_check.weekday().number_from_monday() <= 5 && !holidays.contains(&date_to_check){
                diff += 1.0;
            }
            date_to_check += chrono::Duration::days(1);
        }

        term.clear_last_lines(1).unwrap();
        println!("Overall workdays remaining until last workday: {}", diff);
        thread::sleep(std::time::Duration::from_secs(1));
    }
}