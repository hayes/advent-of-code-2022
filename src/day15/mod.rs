use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

pub fn parse_cords(input: &str) -> (i64, i64) {
    let mut parts = input.split(", ");

    let x = parts
        .next()
        .unwrap()
        .trim_start_matches("x=")
        .parse::<i64>()
        .unwrap();
    let y = parts
        .next()
        .unwrap()
        .trim_start_matches("y=")
        .parse::<i64>()
        .unwrap();

    (x, y)
}

pub fn day15() {
    let mut input: Vec<((i64, i64), (i64, i64), i64)> = INPUT
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");

            let sensor = parts
                .next()
                .unwrap()
                .trim_start_matches("Sensor at ")
                .trim();
            let beacon = parts
                .next()
                .unwrap()
                .trim_start_matches("closest beacon is at ")
                .trim();

            (parse_cords(sensor), parse_cords(beacon))
        })
        .map(|(sensor, beacon)| {
            (
                sensor,
                beacon,
                i64::abs(sensor.0 - beacon.0) + i64::abs(sensor.1 - beacon.1),
            )
        })
        .collect();

    // let part_1 = get_ranges_for_row(2000000, &mut input);
    let part_1 = get_ranges_for_row(10, &mut input);
    let max_range = 4000000;

    let mut i: i64 = 0;
    let mut part_2: i64 = 0;

    loop {
        if i > max_range {
            break;
        }

        let (min, max, _beacons, gap, min_overlap) = get_ranges_for_row(i, &mut input);

        match gap {
            Some(g) => {
                part_2 = g as i64 * 4000000 + i;
                println!("{} {}", g, i);
                break;
            }
            None => {}
        }

        i += i64::min(
            i64::max(1, min_overlap / 2),
            i64::min(-min, max - max_range),
        );
    }

    for i in -3..30 {
        print!("{:3}", i);
    }
    println!("");

    println!(
        "Day 15: {}, {}",
        1 + part_1.1 - part_1.0 - part_1.2 as i64,
        part_2
    );
}

fn get_ranges_for_row(
    row: i64,
    input: &mut Vec<((i64, i64), (i64, i64), i64)>,
) -> (i64, i64, usize, Option<i64>, i64) {
    let mut beacons_on_line: HashSet<i64> = HashSet::new();

    let mut min = get_min(row, *input.first().unwrap());
    let mut max = get_max(row, *input.first().unwrap());
    let mut min_overlap = i64::max_value();
    let mut gap: Option<i64> = None;

    input.sort_by(|a, b| get_min(row, *a).cmp(&get_min(row, *b)));

    for (sensor, beacon, distance) in input.iter() {
        if beacon.1 == row {
            beacons_on_line.insert(beacon.0);
        }

        let md = distance - i64::abs(sensor.1 - row);
        if md < 0 {
            continue;
        };

        min_overlap = i64::min(min_overlap, max - (sensor.0 - md));

        if gap.is_none() && min_overlap < -1 {
            gap = Some(max + 1);
        }

        if sensor.0 - md < min {
            min = sensor.0 - md;
        }

        if sensor.0 + md > max {
            max = sensor.0 + md;
        }
    }

    (min, max, beacons_on_line.len(), gap, min_overlap)
}

fn get_min(row: i64, (sensor, _beacon, distance): ((i64, i64), (i64, i64), i64)) -> i64 {
    let md = distance - i64::abs(sensor.1 - row);
    sensor.0 - md
}

fn get_max(row: i64, (sensor, _beacon, distance): ((i64, i64), (i64, i64), i64)) -> i64 {
    let md = distance - i64::abs(sensor.1 - row);
    sensor.0 + md
}
