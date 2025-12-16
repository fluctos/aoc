use std::error::Error;
use std::process::ExitCode;

fn solve(data: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let manifold: Vec<Vec<char>> = data
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let width = manifold.get(0).map(Vec::len).unwrap_or(0) as i64;

    let mut num_splits = 0u64;
    let mut timelines = vec![0u64; width as usize];

    for row in manifold {
        for (x, chr) in row.iter().enumerate() {
            match chr {
                'S' => {
                    timelines[x] = 1;
                },
                '.' => { },
                '^' => {
                    if x == 0 || x == timelines.len() - 1 {
                        return Err("Manifold splits beam out of the grid".into());
                    }
                    if timelines[x] > 0 {
                        num_splits += 1;
                        timelines[x - 1] += timelines[x];
                        timelines[x + 1] += timelines[x];
                        timelines[x] = 0;
                    }
                },
                _ => {
                    return Err("Malformed input".into());
                },
            }
        }
    }

    Ok((num_splits, timelines.iter().sum()))
}

fn solve_part_one(data: &str) -> Result<u64, Box<dyn Error>> {
    solve(data).map(|(num_splits, _)| num_splits)
}

fn solve_part_two(data: &str) -> Result<u64, Box<dyn Error>> {
    solve(data).map(|(_, num_timelines)| num_timelines)
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("day07/input.txt")?;
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

    const TEST_INPUT: &str = concat!(
        ".......S.......\n",
        "...............\n",
        ".......^.......\n",
        "...............\n",
        "......^.^......\n",
        "...............\n",
        ".....^.^.^.....\n",
        "...............\n",
        "....^.^...^....\n", 
        "...............\n",
        "...^.^...^.^...\n",
        "...............\n",
        "..^...^.....^..\n",
        "...............\n",
        ".^.^.^.^.^...^.\n",
        "...............\n",
    );

    #[test]
    fn day07_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap(), 21);
    }

    #[test]
    fn day07_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap(), 40);
    }
}
