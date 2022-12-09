use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const LINE_LENGTH: i64 = 1_000;
const START: i64 = (LINE_LENGTH * LINE_LENGTH / 2) + LINE_LENGTH / 2;

enum Move {
    Horizontal(i64),
    Vertical(i64),
}

pub fn day9() {
    let moves: Vec<Move> = INPUT
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut parts = line.split(" ");
            let direction = parts.next().unwrap();
            let distance = parts.next().unwrap().parse::<i64>().unwrap();

            match direction {
                "R" => Move::Horizontal(distance),
                "L" => Move::Horizontal(-distance),
                "U" => Move::Vertical(distance),
                "D" => Move::Vertical(-distance),
                _ => panic!("Unknown direction"),
            }
        })
        .collect();

    let mut head: i64 = START;
    let mut tail: i64 = head;

    let mut rope = vec![head; 9];
    let mut visited_1: HashSet<i64> = HashSet::new();
    let mut visited_2: HashSet<i64> = HashSet::new();
    visited_2.insert(tail);

    for m in moves {
        let distance;
        let size;

        match m {
            Move::Horizontal(d) => {
                distance = d;
                size = 1;
            }
            Move::Vertical(d) => {
                distance = d;
                size = LINE_LENGTH;
            }
        }

        for _ in 0..distance.abs() {
            head += if distance > 0 { size } else { -size };
            let new_tail = calculate_position(head, tail);

            if new_tail != tail {
                visited_1.insert(new_tail);
                tail = new_tail
            }

            let mut current = head;

            for i in 0..9 {
                let knot = rope[i];
                let new_location = calculate_position(current, knot);

                if new_location != knot {
                    rope[i] = new_location;
                    current = new_location;
                } else {
                    break;
                }
            }
            visited_2.insert(rope[8]);
        }
    }

    println!("Day 9: {} {}", visited_1.len(), visited_2.len());
}

fn calculate_position(head: i64, tail: i64) -> i64 {
    let head_h = head % LINE_LENGTH;
    let tail_h: i64 = tail % LINE_LENGTH;
    let head_v = head / LINE_LENGTH;
    let tail_v: i64 = tail / LINE_LENGTH;

    let h_offset = head_h - tail_h;
    let v_offset = head_v - tail_v;

    let min_h = if v_offset > 1 || v_offset < -1 { 0 } else { 1 };
    let min_v = if h_offset > 1 || h_offset < -1 { 0 } else { 1 };

    let mut offset = 0;

    if h_offset > min_h {
        offset += 1;
    } else if h_offset < -min_h {
        offset -= 1;
    }

    if v_offset > min_v {
        offset += LINE_LENGTH;
    } else if v_offset < -min_v {
        offset -= LINE_LENGTH;
    }

    tail + offset
}
