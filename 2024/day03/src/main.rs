use regex::Regex;

fn solve_part_one(data: &str) -> u64 {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    regex
        .captures_iter(data)
        .map(|c| c.extract())
        .map(|(_, [op_a, op_b])| (op_a.parse::<u64>().unwrap(), op_b.parse::<u64>().unwrap()))
        .fold(0, |acc, (op_a, op_b)| acc + op_a * op_b)
}

fn solve_part_two(data: &str) -> u64 {
    let regex = Regex::new(r"(do)\(\)|(don't)\(\)|(mul)\(([0-9]+),([0-9]+)\)").unwrap();

    let mut enabled = true;
    let mut result = 0u64;

    for mat in regex.captures_iter(data) {
        if let Some(_do_match) = mat.get(1) { enabled = true; }
        if let Some(_dont_match) = mat.get(2) { enabled = false; }
        if let Some(_mul_match) = mat.get(3) {
            if enabled {
                let op_a = mat.get(4).unwrap().as_str().parse::<u64>().unwrap();
                let op_b = mat.get(5).unwrap().as_str().parse::<u64>().unwrap();
                result += op_a * op_b;
            }
        }
    }

    result
}

fn main() {
    let data = std::fs::read_to_string("day03/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(&data));
    println!("Part two: {}", solve_part_two(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_PART_ONE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))do()don't()";
    const INPUT_PART_TWO: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(INPUT_PART_ONE), 161);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part_two(INPUT_PART_TWO), 48);
    }

}
