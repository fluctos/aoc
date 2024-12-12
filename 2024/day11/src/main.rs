use std::collections::HashMap;

fn count_digits(number: u64) -> u32 {
    let mut digits = 1_u32;
    let mut num = number;

    while num / 10 > 0 {
        digits += 1;
        num /= 10;
    }

    digits
}

fn split_number(number: u64) -> (u64, u64) {
    let exponent = (count_digits(number) + 1) / 2;
    let mask = 10u64.pow(exponent);

    (number / mask, number % mask)
}

fn count_stones(stone: (u64, u64), cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    let (number, steps) = stone;

    if steps == 0 {
        return 1;
    }

    if let Some(&num_stones) = cache.get(&stone) {
        return num_stones
    }

    let num_stones = match number {
        0 => count_stones((1, steps - 1), cache),
        n => match count_digits(n) % 2 {
            0 => {
                let (left, right) = split_number(n);
                count_stones((left, steps - 1), cache) + count_stones((right, steps - 1), cache)
            },
            1 => count_stones((n * 2024, steps - 1), cache),
            _ => unreachable!(),
        },
    };

    cache.insert(stone, num_stones);

    num_stones
}

fn solve(data: &str, steps: u64) -> u64 {
    let stones = data
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|num| (num.parse::<u64>().unwrap(), steps))
        .collect::<Vec<_>>();

    // Part two pushes cache towards ~130k entries
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    stones
        .into_iter()
        .fold(0_u64, |acc, stone| acc + count_stones(stone, &mut cache))
}

fn solve_part_one(data: &str) -> u64 {
    solve(data, 25)
}

fn solve_part_two(data: &str) -> u64 {
    solve(data, 75)
}

fn main() {
    let data = std::fs::read_to_string("day11/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn day11_part_one() {
        assert_eq!(solve_part_one(INPUT), 55312);
    }

    #[test]
    fn day11_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(12), 2);
        assert_eq!(count_digits(123456789), 9);
    }

    #[test]
    fn day11_split_number() {
        assert_eq!(10u64.pow(0), 1);
        assert_eq!(split_number(1), (0, 1));
        assert_eq!(split_number(10), (1, 0));
        assert_eq!(split_number(99), (9, 9));
        assert_eq!(split_number(1000), (10, 0));
        assert_eq!(split_number(11111), (11, 111));
    }
}
