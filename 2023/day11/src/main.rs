use std::collections::HashSet;
use std::cmp::{min, max};

fn calc_distance(src_galaxy: &(usize, usize), dst_galaxy: &(usize, usize), empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>, expansion_rate: usize) -> usize {
    let max_row = max(src_galaxy.0, dst_galaxy.0);
    let min_row = min(src_galaxy.0, dst_galaxy.0);

    let max_col = max(src_galaxy.1, dst_galaxy.1);
    let min_col = min(src_galaxy.1, dst_galaxy.1);

    let num_empty_rows = empty_rows.iter().filter(|&r| *r > min_row && *r < max_row).count();
    let num_empty_cols = empty_cols.iter().filter(|&c| *c > min_col && *c < max_col).count();

    let expanded_rows = num_empty_rows * expansion_rate;
    let expanded_cols = num_empty_cols * expansion_rate;

    max_row - min_row - num_empty_rows + expanded_rows + max_col - min_col - num_empty_cols + expanded_cols
}

fn calc_distances(galaxies: &Vec<(usize, usize)>, empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>, expansion_rate: usize) -> Vec<(usize, usize, usize)> {
    let mut distances: Vec<(usize, usize, usize)> = Vec::new();
    let mut candidates = galaxies.clone();

    while candidates.len() > 0 {
        let src_index = candidates.len() - 1;
        let src_galaxy = candidates.pop().unwrap();
        for (dst_index, dst_galaxy) in candidates.iter().enumerate() {
            distances.push((src_index, dst_index, calc_distance(&src_galaxy, &dst_galaxy, empty_rows, empty_cols, expansion_rate)))
        }
    }

    distances
}

fn solution(input_file_path: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let size = data.lines().next().unwrap().len();

    let mut empty_rows: HashSet<usize> = (0..size).collect();
    let mut empty_cols: HashSet<usize> = (0..size).collect();

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (row, line) in data.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            match chr {
                '#' => {
                    empty_rows.remove(&row);
                    empty_cols.remove(&col);
                    galaxies.push((row, col));
                }
                _ => (),
            }
        }
    }

    let distances_2 = calc_distances(&galaxies, &empty_rows, &empty_cols, 2);
    let distances_1000000 = calc_distances(&galaxies, &empty_rows, &empty_cols, 1000000);

    (distances_2.iter().map(|d| d.2).sum(), distances_1000000.iter().map(|d| d.2).sum())
}

fn main() {
    println!("{:?}", solution("day11/input/test.txt"));
    println!("{:?}", solution("day11/input/input.txt"));
}

