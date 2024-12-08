use itertools::Itertools;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
    Con,
}

fn get_equations<'a>(data: &'a str) -> impl Iterator<Item = (u64, Vec<u64>)> + 'a {
    data.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(target, operands)| (
            target.parse().unwrap(),
            operands
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        ))
}

fn concatenate(acc: u64, operand: u64) -> u64 {
    let exponent: u32 = f64::log10(operand as f64).floor() as u32 + 1;
    acc * 10u64.pow(exponent) + operand
}

fn compute(operands: &Vec<u64>, operators: Vec<&Operator>) -> u64 {
    assert!(operands.len() > 0);
    assert!(operands.len() - 1 == operators.len());
    let mut acc = operands[0];
    for (operand, operator) in operands.iter().skip(1).zip(operators.iter()) {
        acc = match operator {
            Operator::Add => acc + operand,
            Operator::Mul => acc * operand,
            Operator::Con => concatenate(acc, *operand),
        }
    }

    acc
}

fn test_equation(target: u64, operands: Vec<u64>, operators: &Vec<Operator>) -> bool {
    assert!(operands.len() > 1);
    for product in std::iter::repeat(operators.iter()).take(operands.len() - 1).multi_cartesian_product() {
        if target == compute(&operands, product) {
            return true;
        }
    }

    false
}

fn solve(data: &str, operators: Vec<Operator>) -> u64 {
    get_equations(data)
        .fold(0, |acc, (target, operands)| {
            acc + if test_equation(target, operands, &operators) {
                target
            } else {
                0
            }
        })
}

fn solve_part_one(data: &str) -> u64 {
    let operators = vec![Operator::Add, Operator::Mul];
    solve(data, operators)
}

fn solve_part_two(data: &str) -> u64 {
    let operators = vec![Operator::Add, Operator::Mul, Operator::Con];
    solve(data, operators)
}

fn main() {
    let data = std::fs::read_to_string("day07/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day07_test_concatenate() {
        assert_eq!(concatenate(123, 456), 123456);
        assert_eq!(concatenate(  1, 234),   1234);
        assert_eq!(concatenate(123,   4),   1234);
    }

    const INPUT: &str = "190: 10 19\n\
                         3267: 81 40 27\n\
                         83: 17 5\n\
                         156: 15 6\n\
                         7290: 6 8 6 15\n\
                         161011: 16 10 13\n\
                         192: 17 8 14\n\
                         21037: 9 7 18 13\n\
                         292: 11 6 16 20";

    #[test]
    fn day07_part_one() {
        assert_eq!(solve_part_one(INPUT), 3749);
    }

    #[test]
    fn day07_part_two() {
        assert_eq!(solve_part_two(INPUT), 11387);
    }
}

