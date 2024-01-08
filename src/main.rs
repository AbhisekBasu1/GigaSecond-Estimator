use time::{OffsetDateTime, Date, Time, Month, PrimitiveDateTime as DateTime};
use std::io;

fn main() {
    println!("A gigasecond is one billion seconds. \n
    This was a concept introduced by Vernor Vinge in his science fiction novel \"A deepness in the sky\" \n 
    When we colonize mars & other planets, if someone says \"Year\", they'd have to specify what they mean. \n
    A year on Mars, Earth, Jupiter? \n Thus, units like gigaseconds might be used in the future to denote the passage of time.");
    println!("This program calculates your gigasecond age. Enter your birthdate (YYYY-MM-DD HH:MM:SS): ");
    let mut birthdate_input = String::new();
    io::stdin().read_line(&mut birthdate_input).unwrap();
    let birthdate_input = birthdate_input.trim();

    // Separate the date and time parts
    let parts: Vec<&str> = birthdate_input.split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Invalid date format. Please use YYYY-MM-DD HH:MM:SS");
    }

    // Parse the date
    let date_parts: Vec<i32> = parts[0].split('-')
                                      .map(|p| p.parse().expect("Invalid date format"))
                                      .collect();
    if date_parts.len() != 3 {
        panic!("Invalid date format. Please use YYYY-MM-DD");
    }
    let date = Date::from_calendar_date(date_parts[0], Month::try_from(date_parts[1] as u8).unwrap(), date_parts[2] as u8)
                    .expect("Invalid date");

    // Parse the time
    let time_parts: Vec<u8> = parts[1].split(':')
                                      .map(|p| p.parse().expect("Invalid time format"))
                                      .collect();
    if time_parts.len() != 3 {
        panic!("Invalid time format. Please use HH:MM:SS");
    }
    let time = Time::from_hms(time_parts[0], time_parts[1], time_parts[2])
                    .expect("Invalid time");

    // Combine into a DateTime
    let birthdate = DateTime::new(date, time);

    // Convert birthdate to OffsetDateTime with zero offset (UTC)
    let birthdate_utc = birthdate.assume_utc();

    // Get the current DateTime in UTC
    let current_datetime = OffsetDateTime::now_utc();

    // Calculate the difference in seconds
    let duration_lived = current_datetime - birthdate_utc;
    let seconds_lived = duration_lived.whole_seconds();

    // Convert to gigaseconds
    let gigaseconds_lived = seconds_lived as f64 / 1_000_000_000f64;

    println!("You have lived approximately {:.2} gigaseconds.", gigaseconds_lived);
}
