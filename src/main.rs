use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let day = env::args().nth(1).unwrap_or("1".to_string());

    match day.as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(),
        "3" => day3::day3(),
        "4" => day4::day4(),
        "5" => day5::day5(),
        _ => println!("Please enter a valid day, saw: {}", day),
    }
}
