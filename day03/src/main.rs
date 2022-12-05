extern crate array_tool;
use std::fs;
use crate::array_tool::vec::Uniq;


fn part1(file_path: &str) -> u32 {
    let alphabet : Vec<char> = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut common = Vec::<char>::new();
    let input = fs::read_to_string(file_path)
        .expect("Error reading file");
    let sacks = input.lines()
        .map(|line| (&line[0..(line.len() / 2)], &line[(line.len() / 2)..]))
        .collect::<Vec<(&str, &str)>>();

    sacks.into_iter().for_each(|(a, b)| {
        for c in Vec::from_iter(a.chars().into_iter()).unique() {
            if b.contains(c) {
                common.push(c);
            }
        }

    });

    let sum = common.into_iter().map(|c| alphabet.iter().position(|a| *a == c).unwrap() as u32 ).sum::<u32>();
    return sum;
}

fn main() {
    assert_eq!(part1("inputs/example"), 157);
    assert_eq!(part1("inputs/input"), 157);
}
