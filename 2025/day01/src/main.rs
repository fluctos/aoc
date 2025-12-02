use std::{error::Error, process::ExitCode};

enum Turn {
    Left(u32),
    Right(u32),
}

impl TryFrom<&str> for Turn {
    type Error = Box<dyn Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (dir_str, steps_str) = s.split_at(1);
        let steps = steps_str.parse::<u32>()?;
        match dir_str {
            "L" => Ok(Turn::Left(steps)),
            "R" => Ok(Turn::Right(steps)),
            _ => Err(format!("Unrecognized direction tag: '{s}'").into()),
        }
    }
}

fn turn_the_dial(position: u8, turn: Turn) -> (u8, u64) {
    let mut zero_crossings = 0u64;
    let steps = match turn {
        Turn::Left(n) => n as u64,
        Turn::Right(n) => n as u64,
    };

    zero_crossings += steps / 100u64;

    let new_pos = match turn {
        Turn::Left(n) => (position as i64 - n as i64).rem_euclid(100) as u8,
        Turn::Right(n) => (position as i64 + n as i64).rem_euclid(100) as u8,
    };

    zero_crossings = match (turn, position, new_pos) {
        (_, _, 0) => zero_crossings + 1,
        (_, 0, _) => zero_crossings,
        (Turn::Left(_), p, np) if np > p => zero_crossings + 1,
        (Turn::Right(_), p, np) if p > np => zero_crossings + 1,
        _ => zero_crossings,
    };

    (new_pos, zero_crossings)
}

fn solve_part_one(data: &str) -> Result<u64, Box<dyn Error>> {
    let (_pos, count) = data
        .lines()
        .try_fold((50u8, 0u64), |(pos, count), line| -> Result<(u8, u64), Box<dyn Error>> {
            let turn: Turn = line.try_into()?;
            let (new_pos, _) = turn_the_dial(pos, turn);
            let new_count = match new_pos {
                0 => count + 1,
                _ => count,
            };
            Ok((new_pos, new_count))
        })?;
    Ok(count)
}

fn solve_part_two(data: &str) -> Result<u64, Box<dyn Error>> {
    let (_pos, count) = data
        .lines()
        .try_fold((50u8, 0u64), |(pos, count), line| -> Result<(u8, u64), Box<dyn Error>> {
            let turn: Turn = line.try_into()?;
            let (new_pos, zero_crossings) = turn_the_dial(pos, turn);
            let new_count = count + zero_crossings;
            Ok((new_pos, new_count))
        })?;
    Ok(count)
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("day01/input.txt").unwrap();
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

    const TEST_INPUT: &str = "L68\n\
                              L30\n\
                              R48\n\
                              L5\n\
                              R60\n\
                              L55\n\
                              L1\n\
                              L99\n\
                              R14\n\
                              L82";

    #[test]
    fn day01_modulo_experiment() {
        assert_eq!(-1i8 % 100, -1);
    }

    #[test]
    fn day01_rotation() {
        assert_eq!(turn_the_dial(11, Turn::Right(8)), (19, 0));
        assert_eq!(turn_the_dial(19, Turn::Left(19)), (0, 1));
        assert_eq!(turn_the_dial(0, Turn::Left(1)), (99, 0));
        assert_eq!(turn_the_dial(99, Turn::Right(1)), (0, 1));
        assert_eq!(turn_the_dial(5, Turn::Left(10)), (95, 1));
        assert_eq!(turn_the_dial(95, Turn::Right(10)), (5, 1));
    }

    #[test]
    fn day01_part_one() {
        assert_eq!(solve_part_one(TEST_INPUT).unwrap_or(0), 3);
    }

    #[test]
    fn day01_part_two() {
        assert_eq!(solve_part_two(TEST_INPUT).unwrap_or(0), 6);
    }
}
