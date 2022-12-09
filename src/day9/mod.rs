use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

enum Move {
    Horizontal(i32),
    Vertical(i32),
}

pub fn day9() {
    let moves: Vec<Move> = INPUT
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut parts = line.split(" ");
            let direction = parts.next().unwrap();
            let distance = parts.next().unwrap().parse::<i32>().unwrap();

            match direction {
                "R" => Move::Horizontal(distance),
                "L" => Move::Horizontal(-distance),
                "U" => Move::Vertical(distance),
                "D" => Move::Vertical(-distance),
                _ => panic!("Unknown direction"),
            }
        })
        .collect();

    let mut rope = vec![(0, 0); 10];
    let mut visited_1: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_2: HashSet<(i32, i32)> = HashSet::new();

    visited_1.insert(rope[0]);
    visited_2.insert(rope[0]);

    for m in moves {
        let (distance, horizontal) = match m {
            Move::Horizontal(d) => (d, true),
            Move::Vertical(d) => (d, false),
        };

        for _ in 0..distance.abs() {
            if horizontal {
                rope[0].0 += if distance > 0 { 1 } else { -1 }
            } else {
                rope[0].1 += if distance > 0 { 1 } else { -1 }
            }

            let mut current = rope[0];

            for i in 1..10 {
                let knot = rope[i];
                let new_location = calculate_position(current, knot);

                if new_location != knot {
                    rope[i] = new_location;
                    current = new_location;
                } else {
                    break;
                }
            }
            visited_1.insert(rope[0]);
            visited_2.insert(rope[9]);
        }
    }

    println!("Day 9: {} {}", visited_1.len(), visited_2.len());
}

fn calculate_position(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let mut new_tail = tail.clone();
    let h_offset = head.0 - tail.0;
    let v_offset = head.1 - tail.1;

    let min_h = if v_offset > 1 || v_offset < -1 { 0 } else { 1 };
    let min_v = if h_offset > 1 || h_offset < -1 { 0 } else { 1 };

    if h_offset > min_h {
        new_tail.0 += 1;
    } else if h_offset < -min_h {
        new_tail.0 -= 1;
    }

    if v_offset > min_v {
        new_tail.1 += 1;
    } else if v_offset < -min_v {
        new_tail.1 -= 1;
    }

    new_tail
}
