use std::{error::Error, process::ExitCode};
use std::str::Chars;


fn get_iters<'a>(data: &'a str) -> Vec<Chars<'a>> {
    data.lines()
        .map(|l| l.chars())
        .collect()
}

fn all_same<F>(chars: &[Option<char>], pred: F) -> bool
where
    F: Fn(&Option<char>) -> bool,
{
    chars.iter().all(|&c| pred(&c))
}

#[derive(Debug)]
enum Op {
    Add,
    Mul
}

fn map_values_horizontal(chars: &[Option<char>], mut values: Vec<u64>) -> Result<Vec<u64>, Box<dyn Error>> {
    if values.len() < chars.len() {
        values.resize(chars.len(), 0u64);
    }

    for (idx, chr) in chars.iter().enumerate() {
        match chr {
            Some(' ') => {},
            Some(c) => {
                let digit = c.to_digit(10).ok_or("Invalid digit")? as u64;
                values[idx] *= 10;
                values[idx] += digit;
            }
            None => {
                return Err("Unexpected None".into());
            }
        };
    }
    Ok(values)
}

fn map_values_vertical(chars: &[Option<char>], mut values: Vec<u64>) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut val = 0u64;
    for chr in chars.iter() {
        match chr {
            Some(' ') => {},
            Some(c) => {
                let digit = c.to_digit(10).ok_or("Invalid digit")? as u64;
                val *= 10;
                val += digit;
            }
            None => {
                return Err("Unexpected None".into());
            }
        };
    }
    values.push(val);
    Ok(values)
}

fn solve<F>(data: &str, mut map_values: F) -> Result<u64, Box<dyn Error>>
where
    F: FnMut(&[Option<char>], Vec<u64>) -> Result<Vec<u64>, Box<dyn Error>>,
{
    let mut iters = get_iters(data);

    let mut finished = false;
    let mut op: Op = Op::Add;
    let mut values: Vec<u64> = Vec::new();
    let mut result = 0u64;

    let mut options: Vec<Option<char>> = vec![None; iters.len()];

    loop {
        for (dst, iter) in options.iter_mut().zip(iters.iter_mut()) {
            *dst = iter.next();
        }

        if !all_same(&options, |o| o.is_some()) && 
           !all_same(&options, |o| o.is_none()) {
            return Err("Iterators out of sync".into());
        }

        if all_same(&options, |o| o.is_none()) {
            finished = true;
        }

        if finished || all_same(&options, |&o| o == Some(' ')) {
            match op {
                Op::Add => result += values.iter().copied().sum::<u64>(),
                Op::Mul => result += values.iter().copied().product::<u64>(),
            }

            values.clear();

            if finished {
                return Ok(result);
            }

            continue;
        }

        if let Some((last, chars)) = options.split_last() {
            match last {
                Some('+') => op = Op::Add,
                Some('*') => op = Op::Mul,
                _ => (),
            };

            values = map_values(chars, values)?;

        } else {
            return Err("Unexpected values".into());
        }
    }
}

fn solve_part_one(data: &str) -> Result<u64, Box<dyn Error>> {
    solve(data, map_values_horizontal)
}

fn solve_part_two(data: &str) -> Result<u64, Box<dyn Error>> {
    solve(data, map_values_vertical)
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("day06/input.txt")?;
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
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  "
    );

    #[test]
    fn day06_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap(), 4277556);
    }

    #[test]
    fn day06_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap(), 3263827);
    }
}
