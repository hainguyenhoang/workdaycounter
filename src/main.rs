use chrono::prelude::*;
use std::thread;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long)]
    holiday_list_file: Option<String>,

    last_workday: NaiveDate
}

fn main() {
    let args = Args::parse();



    let mut diff = 0;
    let mut date_to_check = Local::now().date_naive();
    while date_to_check < args.last_workday {
        if date_to_check.weekday().number_from_monday() <= 5{
            diff += 1;
        }
        date_to_check += chrono::Duration::days(1);
    }

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Overall workdays remaining until last workday: {}", diff);
        thread::sleep(std::time::Duration::from_secs(1));
    }
}