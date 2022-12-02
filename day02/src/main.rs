use std::fs;

fn part1(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Unable to read file");
    let mut score = 0;

    input.split('\n').for_each(|round| {
        if let &[a, b] = round.split_whitespace().collect::<Vec<&str>>().as_slice() {
            println!("{} VS {}", a, b);
            match b {
                "X" => {
                    score += 1;
                    match a {
                        "A" => score += 3,
                        "C" => score += 6,
                        _ => (),
                    }
                }
                "Y" => {
                    score += 2;
                    match a {
                        "B" => score += 3,
                        "A" => score += 6,
                        _ => (),
                    }
                }
                "Z" => {
                    score += 3;
                    match a {
                        "C" => score += 3,
                        "B" => score += 6,
                        _ => (),
                    }
                }
                _ => ()
            }
        }
    });
    return score;
}

fn main() {
    assert_eq!(part1("./inputs/example"), 15);
    assert_eq!(part1("./inputs/input"), 11449);
}
