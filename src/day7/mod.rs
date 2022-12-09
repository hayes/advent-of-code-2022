use std::collections::HashMap;
use std::slice::Iter;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum InputLine {
    Ls,
    Cd(String),
    File(u32),
    Dir(String),
}

fn parse_line(line: &str) -> Option<InputLine> {
    if line.starts_with("$ ls") {
        return Some(InputLine::Ls);
    } else if line.starts_with("$ cd ") {
        let path = line[5..].trim().to_string();
        return Some(InputLine::Cd(path));
    } else if line.starts_with("dir ") {
        let path = line[4..].trim().to_string();
        return Some(InputLine::Dir(path));
    } else {
        let size = line
            .trim()
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        return Some(InputLine::File(size));
    }
}

pub fn day7() {
    let input = INPUT
        .lines()
        .map(|l| parse_line(l))
        .filter(|l| l.is_some())
        .map(|l| l.unwrap())
        .collect::<Vec<InputLine>>();

    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let root_size = parse_dir(&mut dir_sizes, &mut input.iter(), "".to_string());

    dir_sizes.insert("/".to_string(), root_size);

    let sum: u32 = dir_sizes
        .iter()
        .filter(|(_, size)| **size <= 100_000)
        .map(|(_, size)| size)
        .sum();

    let best = dir_sizes
        .iter()
        .filter(|(_, size)| **size >= 30_000_000 + dir_sizes.get("/").unwrap() - 70_000_000)
        .map(|(_, size)| size)
        .min()
        .unwrap();

    println!("Day 7: {}, {:?}", sum, best);
}

fn parse_dir(map: &mut HashMap<String, u32>, lines: &mut Iter<InputLine>, path: String) -> u32 {
    let mut sum = 0;

    while let Some(line) = lines.next() {
        match line {
            InputLine::Ls => continue,
            InputLine::Dir(_) => continue,
            InputLine::Cd(dir) => match dir.as_str() {
                "/" => continue,
                ".." => return sum,
                _ => {
                    let new_path = format!("{}/{}", path, dir.clone());
                    let size = parse_dir(map, lines, new_path.clone());

                    map.insert(new_path.clone(), size);

                    sum += size;
                }
            },
            InputLine::File(size) => {
                sum += size;
            }
        }
    }

    return sum;
}
