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

fn count_stones(stone: (u64, u64), mut cache: HashMap<(u64, u64), u64>) -> (u64, HashMap<(u64, u64), u64>) {
    if let Some(&num_stones) = cache.get(&stone) {
        return (num_stones, cache)
    }

    let (number, steps) = stone;

    if steps == 0 {
        return (1, cache);
    }

    let (num_stones, tmp_cache) = match number {
        0 => count_stones((1, steps - 1), cache),
        n => match count_digits(n) % 2 {
            0 => {
                let (left, right) = split_number(n);
                let (l_count, tmp_cache) = count_stones((left, steps - 1), cache);
                let (r_count, tmp_cache) = count_stones((right, steps - 1), tmp_cache);
                (l_count + r_count, tmp_cache)
            },
            1 => count_stones((n * 2024, steps - 1), cache),
            _ => unreachable!(),
        },
    };

    cache = tmp_cache;
    cache.insert(stone, num_stones);

    (num_stones, cache)
}

fn solve(data: &str, steps: u64) -> u64 {
    let stones = data
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|num| (num.parse::<u64>().unwrap(), steps))
        .collect::<Vec<_>>();

    // Part two pushes cache towards ~130k entries
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    let mut result = 0u64;
    let mut num_stones;

    for stone in stones {
        (num_stones, cache) = count_stones(stone, cache);
        result += num_stones;
    }

    result
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
