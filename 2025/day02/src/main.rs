use std::{error::Error, process::ExitCode};

fn get_digits(number: u64) -> Vec<u8> {
    match number {
        0 => vec![0],
        num => {
            let mut v = vec![];
            let mut n = num;
            while n != 0 {
                v.push((n % 10) as u8);
                n /= 10;
            }
            v
        }
    }
}

#[derive (Debug, PartialEq)]
enum SymmetryType {
    Half,
    Any
}

fn has_symmetry(number: u64, sym_type: SymmetryType) -> bool {
    let digits = get_digits(number);
    let num_digits = digits.len();

    if sym_type == SymmetryType::Half && num_digits % 2 != 0 {
        return false;
    }

    let mut range = match sym_type {
        SymmetryType::Half => num_digits / 2 ..= num_digits / 2,
        SymmetryType::Any => 1 ..= num_digits / 2,
    };

    range.any(|seq| {
        digits
            .chunks(seq)
            .map(|chunk| (chunk, true))
            .reduce(|(chunk, all_equal), (other, _)| (chunk, all_equal && chunk == other))
            .is_some_and(|(_, all_equal)| all_equal)
    })
}

fn solve_part_one(data: &str) -> Result<u64, Box<dyn Error>> {
    data.split(',')
        .try_fold(0_u64, |acc, rangestr| {
            let (minstr, maxstr) = rangestr
                .split_once('-')
                .ok_or(format!("Missing '-' in string '{rangestr}'"))?;

            let min = minstr.trim().parse::<u64>()?;
            let max = maxstr.trim().parse::<u64>()?;

            let sum: u64 = (min..=max)
                .filter(|&num| has_symmetry(num, SymmetryType::Half))
                .sum();

            Ok(acc + sum)
        })
}

fn solve_part_two(data: &str) -> Result<u64, Box<dyn Error>> {
    data.split(',')
        .try_fold(0_u64, |acc, rangestr| {
            let (minstr, maxstr) = rangestr
                .split_once('-')
                .ok_or(format!("Missing '-' in string '{rangestr}'"))?;

            let min = minstr.trim().parse::<u64>()?;
            let max = maxstr.trim().parse::<u64>()?;

            let sum: u64 = (min..=max)
                .filter(|&num| has_symmetry(num, SymmetryType::Any))
                .sum();

            Ok(acc + sum)
        })
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("day02/input.txt")?;
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

    #[test]
    fn day02_get_digits() {
        assert_eq!(get_digits(0), vec![0]);
        assert_eq!(get_digits(1), vec![1]);
        assert_eq!(get_digits(12), vec![2, 1]);
        assert_eq!(get_digits(123), vec![3, 2, 1]);
    }

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                              1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                              824824821-824824827,2121212118-2121212124";

    #[test]
    fn day02_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap_or(0), 1227775554);
    }

    #[test]
    fn day02_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap_or(0), 4174379265);
    }
}
