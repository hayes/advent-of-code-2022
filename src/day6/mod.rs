const INPUT: &str = include_str!("input.txt");

pub fn day6() {
    let packet = get_packet();
    let message = get_message();

    println!("Day 6: {} {}", packet, message);
}

pub fn get_packet() -> usize {
    let chars = INPUT.trim().chars().collect::<Vec<char>>();

    let mut last_4 = [0 as char; 4];

    for (i, c) in chars.iter().enumerate() {
        last_4[i % 4] = *c;

        if i < 4 {
            continue;
        }

        let mut found_token = true;
        for i in 0..3 {
            for j in (i + 1)..4 {
                if last_4[i] == last_4[j] {
                    found_token = false;
                    break;
                }
            }

            if !found_token {
                break;
            }
        }

        if found_token {
            return i + 1;
        }
    }

    return chars.len();
}

pub fn get_message() -> usize {
    let chars = INPUT.trim().chars().collect::<Vec<char>>();

    let mut last_14 = [0 as char; 14];

    for (i, c) in chars.iter().enumerate() {
        last_14[i % 14] = *c;

        if i < 14 {
            continue;
        }

        let mut found_token = true;
        for i in 0..13 {
            for j in (i + 1)..14 {
                if last_14[i] == last_14[j] {
                    found_token = false;
                    break;
                }
            }

            if !found_token {
                break;
            }
        }

        if found_token {
            return i + 1;
        }
    }

    return chars.len();
}
