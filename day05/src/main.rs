use std::collections::VecDeque;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = Vec::from_iter(s.split_whitespace().into_iter());
        let quantity = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        return Ok(Move { quantity, from, to });
    }
}

#[derive(Debug)]
struct Instructions {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

fn input(file_path: &str) -> Instructions {
    let input = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    for line in input.lines() {
        if line.contains("[") {
            Vec::from_iter(line.chars().into_iter())
                .chunks(4)
                .map(|e| e[1])
                .enumerate()
                .for_each(|(i, e)| {
                    if stacks.len() <= i {
                        stacks.push(VecDeque::new());
                    }

                    if e != ' ' {
                        stacks[i].push_back(e);
                    }
                });
        } else if line.contains("move") {
            let action = Move::from_str(line).unwrap();
            moves.push(action);
        }
    }
    return Instructions { stacks, moves };
}

fn part1(file_path: &str) -> String {
    let mut instructions = input(file_path);

    for action in instructions.moves {
        for _ in 0..action.quantity {
            let crt = instructions.stacks[action.from].pop_front().unwrap();
            instructions.stacks[action.to].push_front(crt);
        }
    }
    return instructions.stacks.into_iter().map(|e| e[0]).collect();
}

fn part2(file_path: &str) -> String {
    let mut instructions = input(file_path);

    for action in instructions.moves {
        for index in 0..action.quantity {
            let crt = instructions.stacks[action.from].pop_front().unwrap();
            instructions.stacks[action.to].insert(index, crt);
        }
    }
    return instructions.stacks.into_iter().map(|e| e[0]).collect();
}

fn main() {
    assert_eq!(part1("inputs/example"), "CMZ");
    assert_eq!(part1("inputs/input"), "JDTMRWCQJ");
    assert_eq!(part2("inputs/example"), "MCD");
    assert_eq!(part2("inputs/input"), "VHJDDCWRD");
}
