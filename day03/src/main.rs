extern crate array_tool;
use std::fs;
use crate::array_tool::vec::{Uniq,Intersect};

#[derive(Debug)]
struct Sack {
    items: Vec<char>,
}

impl Sack {
    fn fst_compartment(&self) -> Vec<char> {
        let middle = self.items.len() / 2;
        let compartment = self.items[0..middle].to_vec();
        return compartment;
    }

    fn scd_compartment(&self) -> Vec<char> {
        let middle = self.items.len() / 2;
        let compartment = self.items[middle..].to_vec();
        return compartment;
    }
}

fn get_sacks(file_path: &str) -> Vec<Sack> {
    let input = fs::read_to_string(file_path)
        .expect("Error reading file");
    input.lines()
        .map(|line| Sack{ items: line.chars().into_iter().collect::<Vec<char>>()})
        .collect()
}

fn items_sum(items: &Vec<char>) -> u32 {
    let alphabet : Vec<char> = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    return items.into_iter().map(|c| alphabet.iter().position(|a| a == c).unwrap() as u32 ).sum::<u32>();
}

fn part1(file_path: &str) -> u32 {
    let mut common = Vec::<char>::new();
    let sacks = get_sacks(file_path);

    for sack in sacks {
        let a = sack.fst_compartment();
        let b = sack.scd_compartment();
        for c in a.unique() {
            if b.contains(&c) {
                common.push(c);
            }
        }

    }
    return items_sum(&common);
}

fn part2(file_path: &str) -> u32 {
    let sacks = get_sacks(file_path);
    let mut sum = 0;

    for group in sacks.chunks(3) {
        let intersect = group[0].items.intersect(group[1].items.clone()).intersect(group[2].items.clone());
        sum += items_sum(&intersect);
    }
    return sum;
}

fn main() {
    assert_eq!(part1("inputs/example"), 157);
    assert_eq!(part1("inputs/input"), 7980);
    assert_eq!(part2("inputs/example"), 70);
    assert_eq!(part2("inputs/input"), 2881);
}
