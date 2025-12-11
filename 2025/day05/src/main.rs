use std::{error::Error, process::ExitCode};

type Range = (u64, u64);

fn are_overlapping(r1: &Range, r2: &Range) -> bool {
    r1.0 <= r2.1 && r1.1 >= r2.0
}

fn range_width(range: &(u64, u64)) -> u64 {
    range.1 - range.0 + 1    
}

fn parse_ranges(data: &str) -> Result<Vec<Range>, Box<dyn Error>> {
    data.lines()
        .map(|line| {
            let tokens = line.split_once('-').ok_or("Expeting '-' in range definition")?;
            let min = tokens.0.parse::<u64>()?;
            let max = tokens.1.parse::<u64>()?;
            if min > max {
                Err("Range min > max")?
            } else {
                Ok((min, max))
            }
        })
        .collect()
}

fn parse_ingredients(data: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    data.lines()
        .map(|line| line.parse::<u64>())
        .map(|r| r.map_err(Into::into))
        .collect()
}

fn solve_part_one(data: &str) -> Result<u64, Box<dyn Error>> {
    let parts = data.split_once("\n\n").ok_or("Malformed input")?;

    let ranges = parse_ranges(parts.0)?;
    let ingredients = parse_ingredients(parts.1)?;

    let num_fresh = ingredients
        .iter()
        .filter(|&&i| ranges.iter().any(|&r| i >= r.0 && i <= r.1))
        .count();

    Ok(num_fresh as u64)
}

fn solve_part_two(data: &str) -> Result<u64, Box<dyn Error>> {
    let parts = data.split_once("\n\n").ok_or("Malformed input")?;
    let mut ranges = parse_ranges(parts.0)?;

    if ranges.is_empty() {
        return Ok(0);
    }
    
    ranges.sort_by_key(|range| range.0);

    let mut width = 0u64;
    let mut current = ranges[0];

    for range in ranges.into_iter().skip(1) {
        if are_overlapping(&current, &range) {
            current = (current.0.min(range.0), current.1.max(range.1));
        } else {
            width += range_width(&current);
            current = range;
        }
    }
            
    width += range_width(&current);
            
    Ok(width)
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("day05/input.txt")?;
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

    const TEST_INPUT: &str = "3-5\n\
                              10-14\n\
                              16-20\n\
                              12-18\n\
                              \n\
                              1\n\
                              5\n\
                              8\n\
                              11\n\
                              17\n\
                              32";

    #[test]
    fn day05_overlaps() {
        assert_eq!(are_overlapping(&(0, 3), &(1, 2)), true);
        assert_eq!(are_overlapping(&(1, 2), &(0, 3)), true);
        assert_eq!(are_overlapping(&(0, 2), &(1, 3)), true);
        assert_eq!(are_overlapping(&(1, 3), &(0, 2)), true);
        assert_eq!(are_overlapping(&(0, 2), &(2, 4)), true);
        assert_eq!(are_overlapping(&(0, 1), &(3, 4)), false);
    }

    #[test]
    fn day05_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap(), 3);
    }

    #[test]
    fn day05_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap(), 14);
    }
}
