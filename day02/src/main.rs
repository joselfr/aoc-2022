use std::fs;

fn part1(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Unable to read file");
    let mut score = 0;

    input.split('\n').for_each(|round| {
        if let &[a, b] = round.split_whitespace().collect::<Vec<&str>>().as_slice() {
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

fn part2(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Unable to read file");
    let mut score = 0;

    input.split('\n').for_each(|round| {
        if let &[a, b] = round.split_whitespace().collect::<Vec<&str>>().as_slice() {
            match b {
                "X" => {
                    match a {
                        "A" => score += 3,
                        "B" => score += 1,
                        "C" => score += 2,
                        _ => (),
                    }
                }
                "Y" => {
                    score += 3;
                    match a {
                        "A" => score += 1,
                        "B" => score += 2,
                        "C" => score += 3,
                        _ => (),
                    }
                }
                "Z" => {
                    score += 6;
                    match a {
                        "A" => score += 2,
                        "B" => score += 3,
                        "C" => score += 1,
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
    assert_eq!(part2("./inputs/example"), 12);
    assert_eq!(part2("./inputs/input"), 13187);
}
