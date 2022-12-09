extern crate array_tool;
use array_tool::vec::Uniq;

fn common(file_path: &str, len: usize) -> u32 {
    let input = std::fs::read_to_string(file_path).unwrap();
    for i in 0..(input.len() - len - 1) {
        let sub = &input[i..(i + len)];
        let chars = sub.chars().collect::<Vec<char>>();

        if chars.unique().len() == len {
            return (i + len) as u32;
        }
    }
    return 0;
}

fn main() {
    assert_eq!(common("inputs/example", 4), 7);
    assert_eq!(common("inputs/input", 4), 1912);
    assert_eq!(common("inputs/example", 14), 19);
    assert_eq!(common("inputs/input", 14), 1912);
}
