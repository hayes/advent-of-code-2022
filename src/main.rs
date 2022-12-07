use std::{env, time::Instant};
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let day = env::args().nth(1).unwrap_or("1".to_string());

    let start = Instant::now();
    match day.as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(),
        "3" => day3::day3(),
        "4" => day4::day4(),
        "5" => day5::day5(),
        "6" => day6::day6(),
        "7" => day7::day7(),
        _ => println!("Please enter a valid day, saw: {}", day),
    }

    println!("Elapsed: {}ms", start.elapsed().as_micros() as f64 / 1000.0);
}
