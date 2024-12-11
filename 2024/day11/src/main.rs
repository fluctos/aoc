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

fn solve(data: &str, max_iterations: usize) -> u64 {
    let mut stones = data
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|num| (0_usize, num.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    let mut num_stones = stones.len();

    while let Some((iterations, mut stone)) = stones.pop() {
        for iter in iterations..max_iterations {
            let num_digits = count_digits(stone);
            stone = match (stone, num_digits % 2) {
                (0, _) => 1,
                (_, 0) => {
                    let (left, right) = split_number(stone);
                    stones.push((iter + 1, right));
                    num_stones += 1;
                    left
                },
                (_, 1) => stone * 2024,
                _ => unreachable!(),
            };
        }
    }

    num_stones as u64
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
    fn day11_part_two() {
        assert_eq!(solve_part_two(INPUT), 0);
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
