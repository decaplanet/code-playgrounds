use std::{io::stdin, vec};

fn check_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            year % 400 == 0
        } else {
            true
        }
    } else {
        false
    }
}

fn days_in_month(year: i32, month: usize, month_days: &[i32]) -> i32 {
    let is_leap_year = check_leap_year(year);

    if is_leap_year {
        if month == 2 {
            29
        } else {
            month_days[month - 1]
        }
    } else {
        month_days[month - 1]
    }
}

fn main() {
    let month_days: Vec<i32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    println!("Enter a year :");
    let mut year = String::new();
    stdin().read_line(&mut year).unwrap();
    let year: i32 = year.trim().parse().unwrap();

    println!("Enter a month :");
    let mut month = String::new();
    stdin().read_line(&mut month).unwrap();
    let month: usize = month.trim().parse().unwrap();

    let days = days_in_month(year, month, &month_days);

    println!("{}", days);
}
