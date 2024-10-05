use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    U {steps: u64},
    R {steps: u64},
    D {steps: u64},
    L {steps: u64},
}

fn solve(dig_plan: Vec<Direction>) -> u64 {
    let mut pos = (0i64, 0i64);
    let mut next;
    let mut area = 0i64;
    let mut perim = 0u64;
    for dir in dig_plan {
        match dir {
            Direction::U{steps} => { next = (pos.0 - steps as i64, pos.1); perim += steps },
            Direction::R{steps} => { next = (pos.0, pos.1 + steps as i64); perim += steps },
            Direction::D{steps} => { next = (pos.0 + steps as i64, pos.1); perim += steps },
            Direction::L{steps} => { next = (pos.0, pos.1 - steps as i64); perim += steps },
        };
        area += pos.0 * next.1;
        area -= pos.1 * next.0;
        pos = next;
    }
    (area.abs() as u64 + perim) / 2 + 1
}

fn parse_data_part1(data: &String) -> Vec<Direction> {
    data.lines().map(|line| {
        let tokens: Vec<_> = line.split(' ').collect();
        let direction = match (tokens[0].chars().nth(0).unwrap(), u64::from_str(tokens[1]).unwrap()) {
            ('U', steps) => Direction::U{steps},
            ('R', steps) => Direction::R{steps},
            ('D', steps) => Direction::D{steps},
            ('L', steps) => Direction::L{steps},
            _ => unreachable!(),
        };
        direction
    }).collect()
}

fn parse_data_part2(data: &String) -> Vec<Direction> {
    data.lines().map(|line| {
        let tokens: Vec<_> = line.split(' ').collect();
        let steps = u64::from_str_radix(&tokens[2][2..7], 16).unwrap();
        match tokens[2][7..8].parse().unwrap() {
            0 => Direction::R{steps},
            1 => Direction::D{steps},
            2 => Direction::L{steps},
            3 => Direction::U{steps},
            _ => unreachable!(),
        }
    }).collect()
}

fn solution(input_file_path: &str) -> (u64, u64) {
    let data = std::fs::read_to_string(input_file_path).unwrap();

    let part_1 = solve(parse_data_part1(&data));
    let part_2 = solve(parse_data_part2(&data));

    (part_1, part_2)
}

fn main() {
    println!("{:?}", solution("day18/input/test.txt"));
    println!("{:?}", solution("day18/input/input.txt"));
}

