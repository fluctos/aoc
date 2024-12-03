use std::collections::{BinaryHeap, HashMap};

fn solve_part_one(data: &str) -> u64 {
    let mut a_values: BinaryHeap<u64> = BinaryHeap::new();
    let mut b_values: BinaryHeap<u64> = BinaryHeap::new();

    for line in data.lines() {
        let mut tokens = line.split_whitespace();
        let a_value = tokens.next().unwrap().parse::<u64>().unwrap();
        let b_value = tokens.next().unwrap().parse::<u64>().unwrap();

        a_values.push(a_value);
        b_values.push(b_value);
    }

    let mut sum_of_dist = 0u64;

    for pair in a_values.into_sorted_vec().into_iter().zip(b_values.into_sorted_vec().into_iter()) {
        sum_of_dist += pair.0.abs_diff(pair.1);
    }

    sum_of_dist
}

fn solve_part_two(data: &str) -> u64 {
    let mut a_values: BinaryHeap<u64> = BinaryHeap::new();
    let mut b_freqs: HashMap<u64, u64> = HashMap::new();

    for line in data.lines() {
        let mut tokens = line.split_whitespace();
        let a_value = tokens.next().unwrap().parse::<u64>().unwrap();
        let b_value = tokens.next().unwrap().parse::<u64>().unwrap();

        a_values.push(a_value);
        b_freqs.entry(b_value).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut similarity = 0u64;

    for a_value in a_values.into_sorted_vec().into_iter() {
        similarity += a_value * *b_freqs.get(&a_value).unwrap_or(&0);
    }

    similarity
}

fn main() {
    let data = std::fs::read_to_string("day01/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(&data));
    println!("Part two: {}", solve_part_two(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day01_test_case() {
        let input = "3   4\n\
                     4   3\n\
                     2   5\n\
                     1   3\n\
                     3   9\n\
                     3   3";

        assert_eq!(solve_part_one(input), 11);
        assert_eq!(solve_part_two(input), 31);
    }
}
