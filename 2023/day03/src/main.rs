use std::collections::HashMap;
use itertools::Itertools;

fn solution(input_file_path: &str) -> (u32, u32) {
    let data = std::fs::read_to_string(input_file_path).unwrap();

    let mut sym_map: HashMap<(i32, i32), (char, Vec<u32>)> = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, symbol) in line.char_indices() {
            match symbol {
                '.' => (),
                c if c.is_digit(10) => (),
                c => {
                    sym_map.entry((row as i32, col as i32)).or_insert((c, vec![]));
                }
            }
        }
    }

    let mut parts_sum = 0u32;
    for (row, line) in data.lines().enumerate() {
        for (_, group) in line.char_indices().group_by(|(_, chr)| chr.is_digit(10)).into_iter().filter(|(key, _)| *key) {
            let digits:Vec<_> = group.collect();
            let number = digits.iter().map(|(_, chr)| chr.to_digit(10).unwrap()).fold(0u32, |acc, d| acc * 10 + d);
            let min_col:i32 = *digits.iter().map(|(idx, _)| idx).min().unwrap() as i32;
            let max_col:i32 = *digits.iter().map(|(idx, _)| idx).max().unwrap() as i32;
            for r in row as i32 - 1 ..= row as i32 + 1 {
                for c in min_col - 1 ..= max_col + 1 {
                    if r == row as i32 && c >= min_col && c <= max_col {
                        continue;
                    }

                    if let Some((_, neighbors)) = sym_map.get_mut(&(r, c)) {
                        neighbors.push(number);
                        parts_sum += number;
                    }
                }
            }
        }
    }

    let ratio_sum:u32 = sym_map
        .values()
        .filter(|(chr, neighbors)| *chr == '*' && neighbors.len() == 2)
        .map(|(_, neighbors)| neighbors.iter().product::<u32>())
        .sum();

    (parts_sum, ratio_sum)
}

fn main() {
    println!("test:  {:?}", solution("day03/input/test.txt"));
    println!("input: {:?}", solution("day03/input/input.txt"));
}

