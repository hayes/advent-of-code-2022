const INPUT: &str = include_str!("input.txt");

pub fn day2() {
    let lines: Vec<Vec<&str>> = INPUT
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();

    let score: i32 = lines
        .iter()
        .map(|line| {
            let opponent = line[0];
            let my_move = line[1];

            match (opponent, my_move) {
                ("A", "X") => 1 + 3,
                ("B", "X") => 1 + 0,
                ("C", "X") => 1 + 6,
                ("A", "Y") => 2 + 6,
                ("B", "Y") => 2 + 3,
                ("C", "Y") => 2 + 0,
                ("A", "Z") => 3 + 0,
                ("B", "Z") => 3 + 6,
                ("C", "Z") => 3 + 3,
                _ => 0,
            }
        })
        .sum();

    let score2: i32 = lines
        .iter()
        .map(|line| {
            let opponent = line[0];
            let my_move = line[1];

            match (opponent, my_move) {
                ("A", "X") => 3 + 0,
                ("B", "X") => 1 + 0,
                ("C", "X") => 2 + 0,
                ("A", "Y") => 1 + 3,
                ("B", "Y") => 2 + 3,
                ("C", "Y") => 3 + 3,
                ("A", "Z") => 2 + 6,
                ("B", "Z") => 3 + 6,
                ("C", "Z") => 1 + 6,
                _ => 0,
            }
        })
        .sum();

    println!("Day 2: {}, {}", score, score2);
}
