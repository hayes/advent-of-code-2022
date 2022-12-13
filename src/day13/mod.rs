use std::cmp::{min, Ordering};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq)]
enum ListItem {
    Value(i32),
    List(Vec<ListItem>),
}

pub fn day13() {
    let pairs: Vec<(ListItem, ListItem)> = INPUT
        .trim()
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();

            let left = parse_list(lines.next().unwrap());
            let right = parse_list(lines.next().unwrap());

            (left, right)
        })
        .collect();

    let mut sum = 0;

    for (i, (left, right)) in pairs.iter().enumerate() {
        let result = compare_list_items(left, right);

        match result {
            Ordering::Less | Ordering::Equal => sum += i + 1,
            _ => continue,
        }
    }

    let extra = vec![
        ListItem::List(vec![ListItem::Value(2)]),
        ListItem::List(vec![ListItem::Value(6)]),
    ];

    let mut packets: Vec<&ListItem> = pairs
        .iter()
        .flat_map(|(left, right)| [left, right])
        .chain([&extra[0], &extra[1]])
        .collect();

    packets.sort_by(|left, right| compare_list_items(left, right));

    let a = packets.iter().position(|p| **p == extra[0]).unwrap() + 1;
    let b = packets.iter().position(|p| **p == extra[1]).unwrap() + 1;

    println!("Day 13: {}, {:?}", sum, a * b);
}

fn compare_list_items(left: &ListItem, right: &ListItem) -> Ordering {
    match (left, right) {
        (ListItem::Value(l), ListItem::Value(r)) => {
            if l < r {
                return Ordering::Less;
            } else if l > r {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        }
        (ListItem::Value(l), ListItem::List(_)) => {
            return compare_list_items(&ListItem::List(vec![ListItem::Value(*l)]), right)
        }
        (ListItem::List(_), ListItem::Value(r)) => {
            return compare_list_items(left, &ListItem::List(vec![ListItem::Value(*r)]))
        }
        (ListItem::List(l), ListItem::List(r)) => {
            for i in 0..min(l.len(), r.len()) {
                match compare_list_items(&l[i], &r[i]) {
                    Ordering::Equal => continue,
                    result => return result,
                }
            }

            return if l.len() <= r.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
    }
}

// Yay co-pilot, this was fully auto-generated
fn parse_list(input: &str) -> ListItem {
    let mut items = Vec::new();
    let mut chars = input.chars();
    let mut current = String::new();

    while let Some(c) = chars.next() {
        match c {
            ' ' => {
                if !current.is_empty() {
                    items.push(ListItem::Value(current.parse().unwrap()));
                    current.clear();
                }
            }
            ',' => {
                if !current.is_empty() {
                    items.push(ListItem::Value(current.parse().unwrap()));
                    current.clear();
                }
            }
            '[' => {
                if !current.is_empty() {
                    items.push(ListItem::Value(current.parse().unwrap()));
                    current.clear();
                }

                let mut depth = 1;
                let mut list = String::new();

                while let Some(c) = chars.next() {
                    match c {
                        '[' => {
                            depth += 1;
                            list.push(c);
                        }
                        ']' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            } else {
                                list.push(c);
                            }
                        }
                        _ => list.push(c),
                    }
                }

                items.push(parse_list(&list));
            }
            ']' => {
                if !current.is_empty() {
                    items.push(ListItem::Value(current.parse().unwrap()));
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        items.push(ListItem::Value(current.parse().unwrap()));
    }

    ListItem::List(items)
}
