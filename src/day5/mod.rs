use regex::Regex;

const INPUT: &str = include_str!("input.txt");

pub fn day5() {
    let stack_re: Regex = Regex::new(r"(?:.(.).(?:.|$))").unwrap();
    let move_re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let input = INPUT.split("\n\n").collect::<Vec<&str>>();

    let stack_lines = input.first().unwrap().lines().collect::<Vec<&str>>();
    let moves: Vec<(usize, usize, usize)> = input
        .get(1)
        .unwrap()
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let cap = move_re.captures_iter(line).next().unwrap();

            let count = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();

            return (count, from - 1, to - 1);
        })
        .collect();

    let mut stacks = stack_lines
        .last()
        .unwrap()
        .split("")
        .filter(|c| c.len() > 0 && *c != " ")
        .map(|_| Vec::<char>::new())
        .collect::<Vec<Vec<char>>>();

    for stack in stack_lines.iter().take(stacks.len() - 1).rev() {
        let cap = stack_re.captures_iter(stack.trim());

        for (j, c) in cap.enumerate() {
            let char = c.get(1).unwrap().as_str().chars().next().unwrap();

            if char == ' ' {
                continue;
            }

            stacks[j].push(char);
        }
    }

    let mut part_2_stacks = stacks.iter().map(|s| s.clone()).collect::<Vec<Vec<char>>>();

    for (count, from, to) in moves.iter() {
        let from_stack = stacks.get_mut(*from).unwrap();
        let mut moved = Vec::<char>::new();

        for _ in 0..*count {
            moved.push(from_stack.pop().unwrap());
        }

        let to_stack = stacks.get_mut(*to).unwrap();
        for c in moved.iter() {
            to_stack.push(*c);
        }
    }

    for (count, from, to) in moves.iter() {
        let from_stack = part_2_stacks.get_mut(*from).unwrap();
        let mut moved = Vec::<char>::new();

        for _ in 0..*count {
            moved.push(from_stack.pop().unwrap());
        }

        let to_stack = part_2_stacks.get_mut(*to).unwrap();
        for c in moved.iter().rev() {
            to_stack.push(*c);
        }
    }

    let tops: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    let tops2: String = part_2_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect();

    println!("Day 5: {}, {}", tops, tops2);
}
