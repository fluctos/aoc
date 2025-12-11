use std::{error::Error, process::ExitCode};

fn calc_joltage(ratings: &[u64]) -> u64 {
    ratings.iter().fold(0u64, |acc, &r| acc * 10 + r)
}

fn calc_joltage_skip_append<const N: usize>(ratings: &[u64; N], skip: usize, append: u64) -> u64 {
    let joltage = ratings
        .iter()
        .enumerate()
        .fold(0u64, |acc, (idx, &rating)| {
            if idx == skip {
                acc
            } else {
                acc * 10 + rating
            }
        });

    joltage * 10 + append
}

fn maximize_perm<const N: usize>(mut perm: [u64; N], new_rating: u64) -> [u64; N] {
    let mut best_joltage = calc_joltage(&perm);
    let mut best_skip_idx: Option<usize> = None;

    for idx in 0..N {
        let candidate = calc_joltage_skip_append(&perm, idx, new_rating);
        if candidate > best_joltage {
            best_joltage = candidate;
            best_skip_idx = Some(idx);
        }
    }

    if let Some(idx) = best_skip_idx {
        perm.copy_within((idx + 1)..N, idx);
        perm[N - 1] = new_rating;
    }

    perm
}

fn solve<const N: usize>(data: &str) -> Result<u64, Box<dyn Error>> {
    data.lines()
        .try_fold(0_u64, |acc, line| {
            // Construct an iterator over the ratings
            let mut ratings = line
                .chars()
                .map(|c| c.to_digit(10).ok_or_else(|| format!("Cannot parse {c}")))
                .map(|r| r.map(|v| v as u64));

            // Prepare initial state
            // Collect N elements into mutable array
            let tmpvec: Vec<u64> = ratings
                .by_ref()
                .take(N)
                .collect::<Result<_, _>>()?;
            let perm: [u64; N] = tmpvec
                .try_into()
                .map_err(|v: Vec<u64>| format!("Expected at least {N} digits, got {}", v.len()))?;

            // Iterate over leftover elements
            // Find permutations giving maximum joltage
            let best_perm = ratings
                .try_fold(perm, |acc, rating| -> Result<[u64; N], Box<dyn Error>> {
                    let r = rating?;
                    let max_perm = maximize_perm(acc, r);
                    Ok(max_perm)
                })?;

            Ok(acc + calc_joltage(&best_perm))
        })
}

fn solve_part_one(data: &str) -> Result<u64, Box<dyn Error>> {
    solve::<2>(data)
}

fn solve_part_two(data: &str) -> Result<u64, Box<dyn Error>> {
    solve::<12>(data) 
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("day03/input.txt")?;
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

    const TEST_INPUT: &str = "987654321111111\n\
                              811111111111119\n\
                              234234234234278\n\
                              818181911112111";

    #[test]
    fn day03_calc_joltage() {
        assert_eq!(calc_joltage(&[1, 2, 3]), 123);
    }

    #[test]
    fn day03_calc_joltage_skip_append() {
        assert_eq!(calc_joltage_skip_append::<3>(&[1, 2, 3], 1, 9), 139);
    }

    #[test]
    fn day03_maximize_perm() {
        assert_eq!(maximize_perm::<3>([3, 1, 2], 1), [3, 2, 1]);
        assert_eq!(maximize_perm::<12>([4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7], 8), [4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]);
    }

    #[test]
    fn day03_copy_within_experiments() {
        let mut array = [0, 1, 2, 3];
        array.copy_within(0..1, 1);
        assert_eq!(array, [0, 0, 2, 3]);
        array.copy_within(0..2, 1);
        assert_eq!(array, [0, 0, 0, 3]);
        array.copy_within(2..4, 0);
        assert_eq!(array, [0, 3, 0, 3]);
    }

    #[test]
    fn day03_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap(), 357);
    }

    #[test]
    fn day03_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap(), 3121910778619);
    }
}
