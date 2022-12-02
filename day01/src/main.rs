use std::{fs,io};
use std::vec::Vec;

fn common(file_path: &str) -> io::Result<Vec<i32>> {
    // read file
    let mut top = Vec::new();
    let mut idx = 0;
    let input = fs::read_to_string(file_path)?;

    for line in input.split('\n') {
        if top.len() <= idx {
            top.push(0);
        }

        match line.parse::<i32>() {
            Ok(num) => top[idx] += num,
            Err(_) => idx += 1
        }
    }
    top.sort();
    return Ok(top);
}

fn part1(file_path: &str) -> i32 {
    match common(file_path) {
        Ok(top) => *top.iter().last().unwrap(),
        Err(e) => panic!("Error: {}", e),
    }
}

fn part2(file_path: &str) -> i32 {
    match common(file_path) {
        Ok(top) => top[(top.len()-3)..].iter().sum(),
        Err(e) => panic!("Error: {}", e),
    }
}

fn main() {
    assert_eq!(part1("./inputs/example"), 24000);
    assert_eq!(part1("./inputs/input"), 66186);
    assert_eq!(part2("./inputs/example"), 45000);
    assert_eq!(part2("./inputs/input"), 196804);
}
