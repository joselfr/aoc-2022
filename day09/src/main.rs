use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    distance: i32,
}

fn parse_input(input_path: &str) -> Vec<Move> {
    fs::read_to_string(input_path)
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance = distance[1..].parse().unwrap();
            match direction {
                "U" => Move {
                    direction: Direction::Up,
                    distance,
                },
                "D" => Move {
                    direction: Direction::Down,
                    distance,
                },
                "L" => Move {
                    direction: Direction::Left,
                    distance,
                },
                "R" => Move {
                    direction: Direction::Right,
                    distance,
                },
                _ => panic!("Unknown direction"),
            }
        })
        .collect()
}

#[allow(dead_code)]
fn debug_grid(knots: &[(i32, i32)]) {
    println!();
    for y in (0..=4).rev() {
        for x in 0..=5 {
            if knots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn move_elems(knots: &mut [(i32, i32)], idx: usize, direction: &Direction) {
    if idx == 0 {
        match direction {
            Direction::Up => knots[idx].1 += 1,
            Direction::Down => knots[idx].1 -= 1,
            Direction::Left => knots[idx].0 -= 1,
            Direction::Right => knots[idx].0 += 1,
        }
    }
    let distance: i32 = ((((knots[idx].0 - knots[idx + 1].0) as i32).pow(2)
        + ((knots[idx].1 - knots[idx + 1].1) as i32).pow(2)) as f32)
        .sqrt()
        .round() as i32;

    // println!("{} ", distance);
    if distance > 1 {
        let mut new_pos_x = (knots[idx].0 + knots[idx + 1].0) as f32 / 2f32;
        let mut new_pos_y = (knots[idx].1 + knots[idx + 1].1) as f32 / 2f32;

        if knots[idx].0 - knots[idx + 1].0 < 0 {
            new_pos_x = new_pos_x.floor();
        } else {
            new_pos_x = new_pos_x.ceil();
        }

        if knots[idx].1 - knots[idx + 1].1 < 0 {
            new_pos_y = new_pos_y.floor();
        } else {
            new_pos_y = new_pos_y.ceil();
        }

        knots[idx + 1].0 = new_pos_x as i32;
        knots[idx + 1].1 = new_pos_y as i32;
    }
}

fn common(file_path: &str, knots: &mut [(i32, i32)]) -> u32 {
    let moves = parse_input(file_path);
    let tail_idx = knots.len() - 1;
    let mut tail_positions: Vec<(i32, i32)> = vec![knots[tail_idx].clone()];

    for m in moves {
        for _ in 0..m.distance {
            for i in 0..tail_idx {
                move_elems(knots, i, &m.direction);
            }
            if !tail_positions.contains(&knots[tail_idx]) {
                tail_positions.push(knots[tail_idx].clone());
            }
        }
    }

    return tail_positions.len() as u32;
}

fn part1(file_path: &str) -> u32 {
    let mut knots = [(0, 0); 2];
    return common(file_path, &mut knots);
}

fn part2(file_path: &str) -> u32 {
    let mut knots = [(0, 0); 10];
    return common(file_path, &mut knots);
}

fn main() {
    assert_eq!(part1("inputs/example"), 13);
    assert_eq!(part1("inputs/input"), 6486);
    assert_eq!(part2("inputs/example"), 1);
    assert_eq!(part2("inputs/input"), 2678);
}
