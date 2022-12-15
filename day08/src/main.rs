fn parse_input(input_path: &str) -> Vec<Vec<u32>> {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }
    return map;
}

fn is_visible(map: &Vec<Vec<u32>>, x: usize, y: usize) -> (bool, u32) {
    let height = map[y][x];
    let mut visible = false;
    let mut left_score = 1;
    let mut right_score = 1;
    let mut up_score = 1;
    let mut down_score = 1;

    // Check left
    for cur_x in (0..=(x - 1)).rev() {
        if map[y][cur_x] >= height {
            break;
        }

        if cur_x == 0 {
            visible = true;
            break;
        }
        left_score += 1;
    }

    // Check right
    for cur_x in (x + 1)..map[y].len() {
        if map[y][cur_x] >= height {
            break;
        }

        if cur_x == map[y].len() - 1 {
            visible = true;
            break;
        }
        right_score += 1;
    }

    // Check up
    for cur_y in (0..=(y - 1)).rev() {
        if map[cur_y][x] >= height {
            break;
        }

        if cur_y == 0 {
            visible = true;
            break;
        }
        up_score += 1;
    }

    // Check down
    for cur_y in (y + 1)..map.len() {
        if map[cur_y][x] >= height {
            break;
        }

        if cur_y == map.len() - 1 {
            visible = true;
            break;
        }
        down_score += 1;
    }

    return (visible, left_score * right_score * up_score * down_score);
}

fn part1(input_path: &str) -> u32 {
    let map = parse_input(input_path);
    let mut count = ((map.len() + map[0].len() - 2) * 2) as u32;

    for y in 1..(map.len() - 1) {
        for x in 1..(map[y].len() - 1) {
            let (visible, _score) = is_visible(&map, x, y);

            if visible {
                count += 1;
            }
        }
    }
    return count;
}

fn part2(input_path: &str) -> u32 {
    let map = parse_input(input_path);
    let mut score: u32 = 0;

    for y in 1..(map.len() - 1) {
        for x in 1..(map[y].len() - 1) {
            let (visible, tree_score) = is_visible(&map, x, y);

            if visible && score < tree_score {
                score = tree_score;
            }
        }
    }
    return score;
}

fn main() {
    assert_eq!(part1("inputs/example"), 21);
    assert_eq!(part1("inputs/input"), 1792);
    assert_eq!(part2("inputs/example"), 8);
    assert_eq!(part2("inputs/input"), 334880);
}
