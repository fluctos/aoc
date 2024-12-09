use std::collections::{HashMap, HashSet};

type Point = (usize, usize);
type Pair = (Point, Point);

fn calc_antinodes_part_one(pairs: Vec<Pair>, max_rows: usize, max_cols: usize) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    let is_point_valid = |point: (i64, i64)| {
        let (row, col) = point;
        row >= 0 && row < max_rows as i64 && col >= 0 && col < max_cols as i64
    };

    for ((a_row, a_col), (b_row, b_col)) in pairs.into_iter() {
        let ab_vec = (b_row as i64 - a_row as i64, b_col as i64 - a_col as i64);
        let ba_vec = (a_row as i64 - b_row as i64, a_col as i64 - b_col as i64);


        let a_res = (b_row as i64 + ab_vec.0, b_col as i64 + ab_vec.1);
        let b_res = (a_row as i64 + ba_vec.0, a_col as i64 + ba_vec.1);

        if is_point_valid(a_res) { antinodes.insert((a_res.0 as usize, a_res.1 as usize)); }
        if is_point_valid(b_res) { antinodes.insert((b_res.0 as usize, b_res.1 as usize)); }
    }

    antinodes
}

fn calc_antinodes_part_two(pairs: Vec<Pair>, max_rows: usize, max_cols: usize) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    let is_point_valid = |point: (i64, i64)| {
        let (row, col) = point;
        row >= 0 && row < max_rows as i64 && col >= 0 && col < max_cols as i64
    };

    for ((a_row, a_col), (b_row, b_col)) in pairs.into_iter() {
        let ab_vec = (b_row as i64 - a_row as i64, b_col as i64 - a_col as i64);

        for multiplier in 1i64.. {
            let point = (a_row as i64 + multiplier * ab_vec.0, a_col as i64 + multiplier * ab_vec.1);

            if !is_point_valid(point) {
                break;
            }

            antinodes.insert((point.0 as usize, point.1 as usize));
        }

        for multiplier in 1i64.. {
            let point = (b_row as i64 - multiplier * ab_vec.0, b_col as i64 - multiplier * ab_vec.1);

            if !is_point_valid(point) {
                break;
            }

            antinodes.insert((point.0 as usize, point.1 as usize));
        }
    }

    antinodes
}

fn get_grid(data: &str) -> Vec<Vec<char>> {
    data.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_antennae_pairs(grid: &Vec<Vec<char>>) -> Vec<Pair> {
    let mut antennae: HashMap<char, Vec<Point>> = HashMap::new();
    let mut pairs: Vec<Pair> = Vec::new();

    for (row, line) in grid.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == '.'{
                continue;
            }

            if let Some(single_freq_antennae) = antennae.get(&char) {
                for antenna in single_freq_antennae {
                    pairs.push((*antenna, (row, col)));
                }
            }

            antennae.entry(*char).or_insert(Vec::new()).push((row, col));
        }
    }

    pairs
}

fn solve(data: &str, calc_func: fn (Vec<Pair>, usize, usize) -> HashSet<Point>) -> u64 {
    let grid = get_grid(data);

    let pairs = get_antennae_pairs(&grid);
    let antinodes = calc_func(pairs, grid.len(), grid[0].len());

    antinodes.len() as u64
}

fn solve_part_one(data: &str) -> u64 {
    solve(data, calc_antinodes_part_one)
}

fn solve_part_two(data: &str) -> u64 {
    solve(data, calc_antinodes_part_two)
}

fn main() {
    let data = std::fs::read_to_string("day08/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "............\n\
                         ........0...\n\
                         .....0......\n\
                         .......0....\n\
                         ....0.......\n\
                         ......A.....\n\
                         ............\n\
                         ............\n\
                         ........A...\n\
                         .........A..\n\
                         ............\n\
                         ............";

    #[test]
    fn day08_part_one() {
        assert_eq!(solve_part_one(INPUT), 14);
    }

    const INPUT_2: &str = "T.........\n\
                           ...T......\n\
                           .T........\n\
                           ..........\n\
                           ..........\n\
                           ..........\n\
                           ..........\n\
                           ..........\n\
                           ..........\n\
                           ..........";

    #[test]
    fn day08_part_two() {
        assert_eq!(solve_part_two(INPUT), 34);
        assert_eq!(solve_part_two(INPUT_2), 9);
    }
}
