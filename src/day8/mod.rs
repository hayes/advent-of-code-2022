use std::cmp::max;

const INPUT: &str = include_str!("input.txt");

pub fn day8() {
    let input: Vec<Vec<i32>> = INPUT
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.split("")
                .filter(|c| c.len() > 0)
                .map(|c| c.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut visible = vec![vec![0; input[0].len()]; input.len()];

    for (i, row) in input.iter().enumerate() {
        let mut t_height = -1;
        let mut b_height = -1;

        for (j, h) in row.iter().enumerate() {
            let h2 = input[i][row.len() - 1 - j];
            if *h > t_height {
                t_height = *h;
                visible[i][j] = 1;
            }

            if h2 > b_height {
                b_height = h2;
                visible[i][row.len() - 1 - j] = 1;
            }
        }
    }

    let col = input.first().unwrap();
    for i in 0..col.len() {
        let mut l_height = -1;
        let mut r_height = -1;

        for j in 0..col.len() {
            let h = input[j][i];
            let h2 = input[col.len() - 1 - j][i];

            if h > l_height {
                l_height = h;
                visible[j][i] = 1;
            }

            if h2 > r_height {
                r_height = h2;
                visible[col.len() - 1 - j][i] = 1;
            }
        }
    }

    let sum: i32 = visible.iter().flat_map(|row| row).sum();

    let part_2 = find_scenic_score(&input);

    println!("Day 8: {}, {}", sum, part_2);
}

// Part 2 as solved by chatGPT
fn find_scenic_score(map: &Vec<Vec<i32>>) -> i32 {
    let mut max_score = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut up = 0;
            let mut down = 0;
            let mut left = 0;
            let mut right = 0;
            let height = map[i][j];

            // Look up
            for k in (0..i).rev() {
                up += 1;
                if map[k][j] >= height {
                    break;
                }
            }

            // Look down
            for k in (i + 1)..map.len() {
                down += 1;
                if map[k][j] >= height {
                    break;
                }
            }

            // Look left
            for k in (0..j).rev() {
                left += 1;
                if map[i][k] >= height {
                    break;
                }
            }

            // Look right
            for k in (j + 1)..map[i].len() {
                right += 1;
                if map[i][k] >= height {
                    break;
                }
            }

            max_score = max(max_score, up * down * left * right);
        }
    }

    max_score
}
