use std::{fs, ops::{RangeInclusive}};

fn string_to_range(s: &str) -> RangeInclusive<u32> {
    let range = s.split('-').map(|s| {
        match s.parse::<u32>() {
            Ok(n) => n,
            Err(_) => panic!("Invalid input"),
        }
    }).collect::<Vec<u32>>();
    return range[0]..=range[1];
}

fn get_input(file_path: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let input = fs::read_to_string(file_path).expect("Error reading file");
    let pairs = input.split('\n').map(|s| {
        match s.split(',').collect::<Vec<&str>>()[..] {
            [a, b] => (string_to_range(a), string_to_range(b)),
            _ => panic!("Invalid input"),
        }
    }).collect::<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>();
    return pairs;
}

fn part1(file_path: &str) -> u32 {
    let pairs = get_input(file_path);
    let count = pairs.iter().filter(|(a,b)| {
        (a.contains(b.start()) && a.contains(b.end())) ||
        (b.contains(a.start()) && b.contains(a.end()))
    }).count() as u32;
    return count;
}

fn part2(file_path: &str) -> u32 {
    let pairs = get_input(file_path);
    let count = pairs.iter().filter(|(a,b)| {
        (a.contains(b.start()) || a.contains(b.end())) ||
        (b.contains(a.start()) || b.contains(a.end()))
    }).count() as u32;
    return count;
}

fn main() {
    assert_eq!(part1("inputs/example"), 2);
    assert_eq!(part1("inputs/input"), 483);
    assert_eq!(part2("inputs/example"), 4);
    assert_eq!(part2("inputs/input"), 483);
}
