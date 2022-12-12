const INPUT: &str = include_str!("input.txt");

pub fn day12() {
    let part_1 = find_path_length(vec!['S']);
    let part_2 = find_path_length(vec!['S', 'a']);

    println!("Day 12: {}, {}", part_1, part_2);
}

pub fn find_path_length(start: Vec<char>) -> i32 {
    let line_length = INPUT.trim().lines().next().unwrap().len();
    let input: Vec<char> = INPUT.trim().chars().filter(|c| *c != '\n').collect();

    let mut heads = input
        .iter()
        .enumerate()
        .filter(|(_, c)| start.contains(*c))
        .map(|(i, _)| (i, 'a'))
        .collect::<Vec<(usize, char)>>();

    let mut distances = vec![-1; input.len()];
    let mut min_steps: i32 = -1;

    for i in 0..input.len() {
        let mut new_heads = vec![];

        for (index, height) in heads {
            let head_height = match input[index] {
                'E' => {
                    if 'z' as i32 - height as i32 > 1 {
                        continue;
                    }

                    distances[index] = i as i32;
                    min_steps = i as i32;
                    break;
                }
                'S' => {
                    if distances[index] != -1 {
                        continue;
                    } else {
                        distances[index] = i as i32;
                    }
                    'a'
                }
                c => {
                    if distances[index] != -1 || c as i32 - height as i32 > 1 {
                        continue;
                    } else {
                        distances[index] = i as i32;
                    }

                    c
                }
            };

            if index > line_length {
                new_heads.push((index - line_length, head_height));
            }
            if index + line_length < input.len() {
                new_heads.push((index + line_length, head_height));
            }
            if index % line_length != 0 {
                new_heads.push((index - 1, head_height));
            }
            if (index + 1) % line_length != 0 {
                new_heads.push((index + 1, head_height));
            }
        }

        if min_steps != -1 || new_heads.len() == 0 {
            break;
        }

        heads = new_heads;
    }

    min_steps
}
