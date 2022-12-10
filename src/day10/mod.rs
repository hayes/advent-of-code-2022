const INPUT: &str = include_str!("input.txt");

enum Instruction {
    NOOP,
    ADDX(i32),
}

pub fn day10() {
    let instructions: Vec<Instruction> = INPUT
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut parts = line.split(" ");
            match parts.next().unwrap() {
                "noop" => Instruction::NOOP,
                "addx" => Instruction::ADDX(parts.next().unwrap().parse::<i32>().unwrap()),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect();

    let mut program = instructions.iter();
    let mut register_x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut sum = 0;
    let mut monitor: Vec<char> = Vec::new();

    loop {
        let (len, increment) = match program.next() {
            Some(Instruction::NOOP) => (1, 0),
            Some(Instruction::ADDX(x)) => (2, *x),
            None => break,
        };

        for _ in 0..len {
            if register_x >= (cycle % 40) - 1 && register_x <= (cycle % 40) + 1 {
                monitor.push('#')
            } else {
                monitor.push('.')
            }

            cycle += 1;

            if (cycle + 20) % 40 == 0 {
                sum += register_x * cycle
            }
        }

        register_x += increment;
    }

    for (i, c) in monitor.iter().enumerate() {
        if i % 40 == 0 {
            println!();
        }

        print!("{}", c);
    }

    println!("");

    println!("Day 10: {} {}", sum, 0);
}
