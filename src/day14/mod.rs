const INPUT: &str = include_str!("input.txt");

pub fn day14() {
    let rocks: Vec<Vec<(u32, u32)>> = INPUT
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|segment| {
                    let mut cords = segment.split(",");

                    (
                        cords.next().unwrap().parse::<u32>().unwrap(),
                        cords.next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let mut rocks_2: Vec<Vec<(u32, u32)>> = rocks.clone();

    let mut x_min: u32 = u32::MAX;
    let mut x_max: u32 = 0;
    let mut y_max: u32 = 0;

    for rock in &rocks {
        for (x, y) in rock {
            if *x < x_min {
                x_min = *x;
            }
            if *x > x_max {
                x_max = *x;
            }
            if *y > y_max {
                y_max = *y;
            }
        }
    }

    let yd = (x_max - x_min) * 2;
    rocks_2.push(vec![(x_max + yd, y_max + 2), (x_min - yd, y_max + 2)]);

    let part_1 = count_sand(&rocks);

    println!(
        "{:?}",
        vec![(x_max + yd, y_max + 2), (x_min - yd, y_max + 2)]
    );
    let part_2 = count_sand(&rocks_2);

    println!("Day 14: {}, {:?}", part_1, part_2);
}

fn count_sand(rocks: &Vec<Vec<(u32, u32)>>) -> i32 {
    let mut x_min: u32 = u32::MAX;
    let mut x_max: u32 = 0;
    let mut y_max: u32 = 0;

    for rock in rocks {
        for (x, y) in rock {
            if *x < x_min {
                x_min = *x;
            }
            if *x > x_max {
                x_max = *x;
            }
            if *y > y_max {
                y_max = *y;
            }
        }
    }

    let mut grid = vec![vec!['.'; (x_max - x_min + 1) as usize]; (y_max + 1) as usize];

    for rock in rocks {
        match rock.len() {
            0 => {}
            1 => {
                let (x, y) = rock[0];
                grid[y as usize][(x - x_min) as usize] = '#';
            }
            _ => {
                let (mut x, mut y) = rock[0];
                grid[y as usize][(x - x_min) as usize] = '#';

                for (x2, y2) in &rock[1..] {
                    while x != *x2 || y != *y2 {
                        if x < *x2 {
                            x += 1;
                        } else if x > *x2 {
                            x -= 1;
                        }
                        if y < *y2 {
                            y += 1;
                        } else if y > *y2 {
                            y -= 1;
                        }
                        grid[y as usize][(x - x_min) as usize] = '#';
                    }
                }
            }
        }
    }

    let mut grain_count = 0;
    'outer: loop {
        let mut grain = (500, 0);

        grain_count += 1;

        if (grid[grain.1][grain.0 - x_min as usize]) == 'o' {
            break;
        }

        loop {
            grain.1 += 1;

            if (grain.1) >= grid.len() {
                break 'outer;
            }

            if grid[grain.1][grain.0 - x_min as usize] == '.' {
                continue;
            }

            if grain.0 == x_min as usize {
                break 'outer;
            }

            if grid[grain.1][(grain.0 - x_min as usize - 1)] == '.' {
                grain.0 -= 1;
                continue;
            }

            if grain.0 == x_max as usize - 1 {
                break 'outer;
            }

            if grid[grain.1][(grain.0 - x_min as usize + 1)] == '.' {
                grain.0 += 1;
                continue;
            }

            grid[grain.1 - 1][grain.0 - x_min as usize] = 'o';
            break;
        }
    }

    for row in &grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }

    return grain_count - 1;
}
