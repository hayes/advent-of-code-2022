use std::{str::FromStr, string::ParseError};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
enum OperationValue {
    Old,
    Value(u32),
}

impl OperationValue {
    fn resolve_value(&self, old: u32) -> u32 {
        match self {
            OperationValue::Old => old,
            OperationValue::Value(v) => *v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(OperationValue, OperationValue),
    Multiply(OperationValue, OperationValue),
}

impl Operation {
    fn calculate(&self, old: u32) -> u32 {
        let (l, r) = match self {
            Operation::Add(l, r) => (l.resolve_value(old), r.resolve_value(old)),
            Operation::Multiply(l, r) => (l.resolve_value(old), r.resolve_value(old)),
        };

        match self {
            Operation::Add(_, _) => l + r,
            Operation::Multiply(_, _) => l * r,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ThrowTargets {
    if_true: u32,
    if_false: u32,
}

#[derive(Debug, Clone)]
struct Monkey {
    offset: usize,
    items: Vec<u32>,
    operation: Operation,
    test: u32,
    targets: ThrowTargets,
}

impl Monkey {
    fn inspect_item(&mut self, worry: u32) -> Option<(u32, u32)> {
        let item = *self.items.get(self.offset)?;
        self.offset += 1;

        let item = self.operation.calculate(item) / worry;

        if item % self.test == 0 {
            Some((self.targets.if_true, item))
        } else {
            Some((self.targets.if_false, item))
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseError;
    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(input: &str) -> Result<Self, ParseError> {
        let mut lines = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| line.len() > 0);

        lines.next();

        let items: Vec<u32> = lines
            .next()
            .unwrap()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(",")
            .map(|item| item.trim().parse::<u32>().unwrap())
            .collect();
        let mut operation_parts = lines
            .next()
            .unwrap()
            .strip_prefix("Operation: new = ")
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        operation_parts.sort();

        let op = *operation_parts.first().unwrap();
        let left = match operation_parts.get(1) {
            Some(&"old") => OperationValue::Old,
            Some(val) => OperationValue::Value(val.parse::<u32>().unwrap()),
            _ => panic!("Unknown operation value"),
        };
        let right = match operation_parts.get(2) {
            Some(&"old") => OperationValue::Old,
            Some(val) => OperationValue::Value(val.parse::<u32>().unwrap()),
            _ => panic!("Unknown operation value"),
        };

        let operation = match op {
            "+" => Operation::Add(left, right),
            "*" => Operation::Multiply(left, right),
            _ => panic!("Unknown operation {}", op),
        };
        let test: u32 = lines
            .next()
            .unwrap()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let if_true = lines
            .next()
            .unwrap()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let if_false = lines
            .next()
            .unwrap()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        Ok(Monkey {
            offset: 0,
            items,
            operation,
            test,
            targets: ThrowTargets { if_true, if_false },
        })
    }
}

pub fn day11() {
    let part_1 = do_monkey_business(3, 20);
    // let part_2 = do_monkey_business(1, 1000);

    println!("Day 11: {} {}", part_1, 0);
}

fn do_monkey_business(worry: u32, rounds: u32) -> u32 {
    let mut monkeys: Vec<Monkey> = INPUT
        .split("\n\n")
        .map(|monkey| monkey.parse::<Monkey>().unwrap())
        .collect();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut thrown: Vec<(u32, u32)> = Vec::new();
            while let Some((target, value)) = monkeys.get_mut(i).unwrap().inspect_item(worry) {
                thrown.push((target, value));
            }
            for (target, value) in thrown {
                monkeys.get_mut(target as usize).unwrap().items.push(value)
            }
        }
    }

    let mut counts: Vec<u32> = monkeys.iter().map(|monkey| monkey.offset as u32).collect();

    counts.sort();

    counts.get(counts.len() - 1).unwrap() * counts.get(counts.len() - 2).unwrap()
}
