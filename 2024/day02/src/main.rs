enum Dampener {
    Disabled,
    Enabled
}

fn are_levels_safe(levels: &Vec<i64>, skip_nth: Option<usize>) -> bool {
    let mut prev: Option<i64> = None;

    let mut increasing = true;
    let mut decreasing = true;

    for (idx, level) in levels.iter().enumerate() {

        if let Some(n) = skip_nth {
            if n == idx {
                continue;
            }
        }

        if let Some(l) = prev {
            let diff = level - l;

            increasing &= diff > 0;
            decreasing &= diff < 0;
            let within = (1..=3).contains(&diff.abs());

            if !(increasing || decreasing) || !within {
                return false;
            }
        }

        prev = Some(*level);
    }

    true
}

fn are_dampened_levels_safe(levels: &Vec<i64>) -> bool {
    for idx in 0..levels.len() {
        if are_levels_safe(levels, Some(idx)) {
            return true
        }
    }

    false
}

fn solve(data: &str, dampener_state: Dampener) -> u64 {
    data
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|v| match dampener_state {
            Dampener::Disabled => are_levels_safe(&v, None),
            Dampener::Enabled  => are_dampened_levels_safe(&v),
        })
        .filter(|&b| b)
        .count() as u64
}

fn main() {
    let data = std::fs::read_to_string("day02/input.txt").unwrap();

    println!("Part one: {}", solve(&data, Dampener::Disabled));
    println!("Part two: {}", solve(&data, Dampener::Enabled));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7 6 4 2 1\n\
                         1 2 7 8 9\n\
                         9 7 6 2 1\n\
                         1 3 2 4 5\n\
                         8 6 4 4 1\n\
                         1 3 6 7 9";

    #[test]
    fn day02_part_one() {
        assert_eq!(solve(INPUT, Dampener::Disabled), 2);
    }

    #[test]
    fn day02_part_two() {
        assert_eq!(solve(INPUT, Dampener::Enabled), 4);
    }
}
