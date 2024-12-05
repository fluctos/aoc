fn solve_part_one(data: &str) -> u64 {
    0
}

fn solve_part_two(data: &str) -> u64 {
    0
}

fn main() {
    let data = std::fs::read_to_string("{{crate_name}}/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn {{crate_name}}_part_one() {
        assert_eq!(solve_part_one(INPUT), 0);
    }

    #[test]
    fn {{crate_name}}_part_two() {
        assert_eq!(solve_part_two(INPUT), 0);
    }
}
