use std::{error::Error, process::ExitCode};
use std::collections::HashSet;

fn parse_grid(data: &str) -> HashSet<(i64, i64)> {
    data.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, chr)| {
                    (chr == '@').then_some((row as i64, col as i64))
                })
        })
        .collect()
}

fn get_num_neighbors(grid: &HashSet<(i64, i64)>, (row, col): (i64, i64)) -> usize {
    const NEIGHBOR_OFFSETS: [(i64, i64); 8] = [
        (-1, -1), (-1, 0), (-1,  1),
        ( 0, -1),          ( 0,  1),
        ( 1, -1), ( 1, 0), ( 1,  1)
    ];

    NEIGHBOR_OFFSETS
        .iter()
        .filter(|&&(dr, dc)| grid.contains(&(row + dr, col + dc)))
        .count()
}

fn solve_part_one(data: &str) -> Result<u64, Box<dyn Error>> {
    let grid = parse_grid(data);

    let num_loose_rolls = grid
        .iter()
        .filter(|&&pos| get_num_neighbors(&grid, pos) < 4)
        .count();

    Ok(num_loose_rolls as u64)
}

fn solve_part_two(data: &str) -> Result<u64, Box<dyn Error>> {
    let mut grid = parse_grid(data);
    let mut removed = 0u64;

    loop {
        let candidates: Vec<(i64, i64)> = grid
            .iter()
            .filter(|&&pos| get_num_neighbors(&grid, pos) < 4)
            .copied()
            .collect();

        if candidates.is_empty() {
            break;
        }

        removed += candidates.len() as u64;

        for pos in &candidates {
            grid.remove(pos);
        }
    }

    Ok(removed)
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("day04/input.txt")?;
    let answer_one = solve_part_one(&data)?;
    let answer_two = solve_part_two(&data)?;

    println!("Part one: {}", answer_one);
    println!("Part two: {}", answer_two);

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.\n\
                              @@@.@.@.@@\n\
                              @@@@@.@.@@\n\
                              @.@@@@..@.\n\
                              @@.@@@@.@@\n\
                              .@@@@@@@.@\n\
                              .@.@.@.@@@\n\
                              @.@@@.@@@@\n\
                              .@@@@@@@@.\n\
                              @.@.@@@.@.";

    #[test]
    fn day04_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn day04_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap(), 43);
    }
}
