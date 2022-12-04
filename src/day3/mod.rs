const INPUT: &str = include_str!("input.txt");

pub fn day3() {
    let bags = INPUT.lines().map(|line| {
        let size = line.len() / 2;
        let mut left = vec!['a'; size];
        let mut right = vec!['a'; size];

        for (i, c) in line.chars().enumerate() {
            if i >= size {
                break;
            }

            left[i] = c;
            right[i] = line.chars().nth(i + size).unwrap();
        }

        (left, right)
    });

    let dups: Vec<Option<(char, u32)>> = bags
        .map(|(left, right)| {
            for (_, l) in left.iter().enumerate() {
                for (_, r) in right.iter().enumerate() {
                    if l == r {
                        if l.is_lowercase() {
                            return Some((l.clone(), *l as u32 - 'a' as u32 + 1));
                        } else {
                            return Some((l.clone(), *l as u32 - 'A' as u32 + 27));
                        };
                    }
                }
            }

            println!("No matching items found for {:?} - {:?}", left, right);

            None
        })
        .collect();

    let mut badge_sum = 0;

    let lines: Vec<&str> = INPUT.split("\n").filter(|line| line.len() > 0).collect();
    let mut i = 0;

    while i < lines.len() {
        println!("{} {}", i, lines.len());
        let a = lines[i];
        let b = lines[i + 1];
        let c = lines[i + 2];

        for (_, d) in a.chars().enumerate() {
            println!("{} {} {} {}", a, b, c, d);
            if b.contains(d) && c.contains(d) {
                badge_sum += if d.is_lowercase() {
                    d as u32 - 'a' as u32 + 1
                } else {
                    d as u32 - 'A' as u32 + 27
                };
                break;
            }
        }

        i += 3
    }

    let sum: u32 = dups.iter().map(|d| d.unwrap().1).sum();

    for (i, d) in dups.iter().enumerate() {
        if let Some((c, n)) = d {
            println!("{}: {} - {}", i, c, n);
        }
    }

    println!("Day 3: {} {}", sum, badge_sum);
}
