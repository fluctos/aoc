use std::iter::zip;

fn get_diffs(a: &Vec<char>, b: &Vec<char>) -> usize {
    zip(a, b).fold(0usize, |acc, pair| acc + match pair.0 == pair.1 {
        true => 0,
        false => 1,
    })
}

fn check_vert_symetry(pattern: &Vec<Vec<char>>, max_diff: usize) -> Option<usize> {
    let mut symmetries: Vec<(usize, usize, usize)> = Vec::new();

    for index in 1..pattern.len() {
        for (_, start_index, _) in symmetries.iter_mut() {
            *start_index -= 1;
        }

        symmetries.push((index, index - 1, 0));

        for (_, start_index, num_diffs) in symmetries.iter_mut() {
            *num_diffs += get_diffs(&pattern[index], &pattern[*start_index]);
        }

        symmetries.retain(|(_, start_index, num_diffs)| {
            match *start_index {
                0 => num_diffs == &max_diff,
                _ => num_diffs <= &max_diff,
            }
        });

        if let Some(i) = symmetries.iter().position(|(_, start_index, num_diffs)| *start_index == 0 && *num_diffs == max_diff) {
            return Some(symmetries[i].0);
        }
    }

    symmetries.iter().filter(|(_, _, num_diffs)| *num_diffs == max_diff).min_by(|a, b| a.1.cmp(&b.1)).map(|s| s.0)
}

fn check_horiz_symetry(pattern: &Vec<Vec<char>>, max_diff: usize) -> Option<usize> {
    let mut iters: Vec<_> = pattern.iter().map(|l| l.iter()).collect();
    let transposed: Vec<Vec<char>> = (0..pattern[0].len())
        .map(|_| iters.iter_mut().map(|iter| *iter.next().unwrap()).collect())
        .collect();

    check_vert_symetry(&transposed, max_diff)
}

fn get_score(pattern: &Vec<Vec<char>>, max_diff: usize) -> usize {
    let mut score = 0usize;

    if let Some(row) = check_vert_symetry(&pattern, max_diff) {
        score += 100 * row;
    }

    if let Some(col) = check_horiz_symetry(&pattern, max_diff) {
        score += col;
    }

    score
}

fn solution(input_file_path: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut part_a = 0usize;
    let mut part_b = 0usize;

    let patterns: Vec<Vec<Vec<char>>> = data
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|line| line.chars().collect()).collect())
        .collect();

    for pattern in patterns {
        part_a += get_score(&pattern, 0);
        part_b += get_score(&pattern, 1);
    }

    (part_a, part_b)
}

fn main() {
    println!("{:?}", solution("day13/input/test.txt"));
    println!("{:?}", solution("day13/input/input.txt"));
}
