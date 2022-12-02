const INPUT: &str = include_str!("input.txt");

pub fn day1() {
    let mut totals: Vec<i64> = INPUT
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<i64>().unwrap()).sum())
        .collect();

    totals.sort_by(|a, b| b.cmp(a));

    let mut sum = 0;

    for (i, total) in totals.iter().enumerate() {
        if i == 3 {
            break;
        }

        sum += total
    }

    println!("Day 1: max={} top3={}", totals[0], sum);
}
