use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct GameDesc {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64)
}

impl GameDesc {
    fn new(button_a: (u64, u64), button_b: (u64, u64), prize: (u64, u64)) -> Self {
        GameDesc{button_a, button_b, prize}
    }

    fn solve(&self) -> Option<(u64, u64)> {
        let [ax, bx, ay, by, px, py]: [i64;6] = [
            self.button_a.0 as i64,
            self.button_b.0 as i64,
            self.button_a.1 as i64,
            self.button_b.1 as i64,
            self.prize.0 as i64,
            self.prize.1 as i64,
        ];

        let det = ax * by - ay * bx;
        let a =  by * px - bx * py;
        let b = -ay * px + ax * py;

        if det == 0 || a % det != 0 || b % det != 0 {
            return None;
        }

        Some(((a / det) as u64, (b / det) as u64))
    }
}


fn parse_button_str(line: &str) -> (u64, u64) {
    line.split_once(": ")
        .unwrap()
        .1
        .split_once(", ")
        .map(|(left, right)| {
            let x_part = left.trim_start_matches("X+").parse::<u64>().unwrap();
            let y_part = right.trim_start_matches("Y+").parse::<u64>().unwrap();
            (x_part, y_part)
        })
        .unwrap()
}

fn parse_prize_str(line: &str) -> (u64, u64) {
    line.trim_start_matches("Prize: ")
        .split_once(", ")
        .map(|(left, right)| {
            let x_part = left.trim_start_matches("X=").parse::<u64>().unwrap();
            let y_part = right.trim_start_matches("Y=").parse::<u64>().unwrap();
            (x_part, y_part)
        })
        .unwrap()
}

impl FromStr for GameDesc {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let button_a = parse_button_str(lines.next().unwrap());
        let button_b = parse_button_str(lines.next().unwrap());
        let prize = parse_prize_str(lines.next().unwrap());
        Ok(GameDesc::new(button_a, button_b, prize))
    }
}

fn solve_part_one(data: &str) -> u64 {
    data.split("\n\n")
        .map(|s| GameDesc::from_str(s).unwrap())
        .filter_map(|gd| gd.solve())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn solve_part_two(data: &str) -> u64 {
    let offset = 10_000_000_000_000;
    data.split("\n\n")
        .map(|s| GameDesc::from_str(s).unwrap())
        .map(|gd| GameDesc{prize: (gd.prize.0 + offset, gd.prize.1 + offset), ..gd})
        .filter_map(|gd| gd.solve())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn main() {
    let data = std::fs::read_to_string("day13/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    const CASE_1: &str = "Button A: X+94, Y+34\n\
                          Button B: X+22, Y+67\n\
                          Prize: X=8400, Y=5400";

    const CASE_2: &str = "Button A: X+26, Y+66\n\
                          Button B: X+67, Y+21\n\
                          Prize: X=12748, Y=12176";

    const CASE_3: &str = "Button A: X+17, Y+86\n\
                          Button B: X+84, Y+37\n\
                          Prize: X=7870, Y=6450";

    const CASE_4: &str = "Button A: X+69, Y+23\n\
                          Button B: X+27, Y+71\n\
                          Prize: X=18641, Y=10279";
    #[test]
    fn day13_parsing() {
        assert_eq!(GameDesc::from_str(CASE_1).unwrap(), GameDesc{button_a: (94, 34), button_b: (22, 67), prize: ( 8400,  5400)});
        assert_eq!(GameDesc::from_str(CASE_2).unwrap(), GameDesc{button_a: (26, 66), button_b: (67, 21), prize: (12748, 12176)});
        assert_eq!(GameDesc::from_str(CASE_3).unwrap(), GameDesc{button_a: (17, 86), button_b: (84, 37), prize: ( 7870,  6450)});
        assert_eq!(GameDesc::from_str(CASE_4).unwrap(), GameDesc{button_a: (69, 23), button_b: (27, 71), prize: (18641, 10279)});
    }

    #[test]
    fn day13_part_one() {
        let data = [CASE_1.to_string(), CASE_2.to_string(), CASE_3.to_string(), CASE_4.to_string()].join("\n\n");
        assert_eq!(solve_part_one(data.as_str()), 480);
    }

    #[test]
    fn day13_part_two() {
        let data = [CASE_1.to_string(), CASE_2.to_string(), CASE_3.to_string(), CASE_4.to_string()].join("\n\n");
        assert_eq!(solve_part_two(data.as_str()), 875318608908);
    }
}
