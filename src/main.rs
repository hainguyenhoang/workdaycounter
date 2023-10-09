use std::fs;
use std::collections::HashSet;
use std::str::FromStr;
use chrono::prelude::*;
use chrono::Duration;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long)]
    /// Optional path to file containing holidays, each in its own line in %Y-%m-%d format e.g.: 2023-10-09
    holiday_path: Option<String>,

    /// Optionally calculate from a given time, in %Y-%m-%dT%H:%M:%S%.f format, e.g.: 2023-10-09T12:00:00.0
    #[arg(long)]
    now: Option<NaiveDateTime>,

    /// The date and time when you end work in %Y-%m-%dT%H:%M:%S%.f format, e.g.: 2023-11-17T16:00:00.0. An 8 hour shift is assumed.
    end_of_work: NaiveDateTime
}

fn parse_holidays(holiday_opt: &Option<String>) -> HashSet<NaiveDate> {
    let mut holidays = HashSet::new();
    if let Some(holiday_path) = holiday_opt {
        if let Ok(holiday_file_string) = fs::read_to_string(holiday_path){
            for holiday_str in holiday_file_string.split('\n') {
                if let Ok(holiday) = NaiveDate::from_str(holiday_str) {
                    holidays.insert(holiday);
                }
            }
        }
    }
    holidays
}

fn calculate_remaining_workdays(now: &NaiveDateTime, end_of_work: &NaiveDateTime,
                                      holidays: &HashSet<NaiveDate>) -> f64{
    let mut ret = 0.0;
    if now.weekday().number_from_monday() <= 5 && !holidays.contains(&now.date()){
        let seconds_remaining_today = (end_of_work.time() - now.time()).num_seconds();
        let today_remaining = seconds_remaining_today as f64 / (8 * 60 * 60) as f64;
        ret += today_remaining.min(1.0).max(0.0);
    }
    let mut date_to_check = now.date() + Duration::days(1);
    while date_to_check <= end_of_work.date() {
        if date_to_check.weekday().number_from_monday() <= 5 && !holidays.contains(&date_to_check){
            ret += 1.0;
        }
        date_to_check += Duration::days(1);
    }
    ret
}

fn main() {
    let args = Args::parse();
    let holidays = parse_holidays(&args.holiday_path);

    let mut now = Local::now().naive_local();
    if let Some(new_now) = args.now {
        now = new_now;
    }
    println!("{:.5}", calculate_remaining_workdays(&now, &args.end_of_work,
                                                                    &holidays));
}