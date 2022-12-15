use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
    size: usize,
}

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

fn process_dir(lines: &mut VecDeque<&str>) -> Dir {
    let mut dir = Dir {
        name: String::new(),
        files: Vec::new(),
        dirs: Vec::new(),
        size: 0,
    };
    let line = lines.pop_front().unwrap();

    if let [_, _, dir_name] = line.split_whitespace().collect::<Vec<&str>>()[..3] {
        dir.name = dir_name.to_string();

        while lines.len() > 0 {
            let args = lines[0].split_whitespace().collect::<Vec<&str>>();
            match args[0] {
                "dir" => (),
                "$" => match args[1] {
                    "cd" => match args[2] {
                        ".." => return dir,
                        _ => {
                            let new_dir = process_dir(lines);
                            dir.size += new_dir.size;
                            dir.dirs.push(new_dir);
                        }
                    },
                    _ => (),
                },
                _ => {
                    if let [file_size, file_name] = args[..2] {
                        let file = File {
                            _name: file_name.to_string(),
                            size: file_size.parse().unwrap(),
                        };
                        dir.size += file.size;
                        dir.files.push(file);
                    }
                }
            }
            lines.pop_front();
        }
    }
    return dir;
}

fn sum_dirs(dirs: &Vec<Dir>) -> usize {
    let mut sum = 0;
    for dir in dirs {
        if dir.size < 100000 {
            sum += dir.size;
        }
        sum += sum_dirs(&dir.dirs);
    }
    return sum;
}

fn part1(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut lines = input.lines().collect::<VecDeque<&str>>();
    let tree = process_dir(&mut lines);
    let sum = sum_dirs(&tree.dirs);
    return sum as u32;
}

fn delete_dirs(dir: &Dir, target_size: usize) -> Option<usize> {
    let mut current_size = None;
    if target_size + dir.size >= 30000000 {
        current_size = Some(dir.size);
    }
    for dir in &dir.dirs {
        match delete_dirs(dir, target_size) {
            Some(size) => {
                if current_size == None || size < current_size.unwrap() {
                    current_size = Some(size);
                }
            }
            None => (),
        }
    }
    return current_size;
}

fn part2(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).unwrap();
    let mut lines = input.lines().collect::<VecDeque<&str>>();
    let tree = process_dir(&mut lines);
    let size = delete_dirs(&tree, 70000000 - tree.size).unwrap();
    return size as u32;
}

fn main() {
    assert_eq!(part1("inputs/example"), 95437);
    assert_eq!(part1("inputs/input"), 1648397);
    assert_eq!(part2("inputs/example"), 24933642);
    assert_eq!(part2("inputs/input"), 1815525);
}
