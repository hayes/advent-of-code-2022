use std::env;
mod day1;
mod day2;

fn main() {
    let day = env::args().nth(1).unwrap_or("1".to_string());

    match day.as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(),
        _ => println!("Please enter a valid day, saw: {}", day),
    }
}
