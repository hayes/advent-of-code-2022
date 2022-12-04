const INPUT: &str = include_str!("input.txt");

pub fn day4() {
    let total: i32 = INPUT
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(",")
                .map(|range| {
                    let parts: Vec<&str> = range.split("-").collect();

                    let min = parts.first().unwrap().parse::<i32>().unwrap();
                    let max = parts.get(1).unwrap().parse::<i32>().unwrap();

                    return (min, max);
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .map(|ranges| {
            let first = ranges.first().unwrap();
            let second = ranges.get(1).unwrap();

            if first.0 <= second.0 && first.1 >= second.1 {
                return 1;
            } else if second.0 <= first.0 && second.1 >= first.1 {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    let partial: i32 = INPUT
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(",")
                .map(|range| {
                    let parts: Vec<&str> = range.split("-").collect();

                    let min = parts.first().unwrap().parse::<i32>().unwrap();
                    let max = parts.get(1).unwrap().parse::<i32>().unwrap();

                    return (min, max);
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .map(|ranges| {
            let first = ranges.first().unwrap();
            let second = ranges.get(1).unwrap();

            if first.0 <= second.0 && first.1 >= second.0 {
                return 1;
            } else if second.0 <= first.0 && second.1 >= first.0 {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("Day 4: {}, {}", total, partial);
}
