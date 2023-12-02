use std::str::FromStr;

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn new() -> Self {
        Self{red: 0, green: 0, blue: 0}
    }
}

impl FromStr for Draw {
    type Err = ();
    fn from_str(input: &str) -> Result<Draw, Self::Err> {
        let mut draw = Draw::new();
        for cube_def in input.trim().split(", ") {
            match cube_def.split_once(' ') {
                Some((n_str, "red")) => draw.red = n_str.parse::<u32>().unwrap(),
                Some((n_str, "green")) => draw.green = n_str.parse::<u32>().unwrap(),
                Some((n_str, "blue")) => draw.blue = n_str.parse::<u32>().unwrap(),
                _ => (),
            }
        }

        Ok(draw)
    }
}

fn playable(draws: &Vec<Draw>) -> bool {
    for draw in draws {
        if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
            return false;
        }
    }

    true
}

fn power(draws: &Vec<Draw>) -> u32 {
    let reds = draws.iter().map(|d| d.red).max().unwrap();
    let greens = draws.iter().map(|d| d.green).max().unwrap();
    let blues = draws.iter().map(|d| d.blue).max().unwrap();

    reds * greens * blues
}

fn solution(path: &str) -> (u32, u32) {
    let data = std::fs::read_to_string(path).unwrap();
    let lines = data.lines();
    let mut sum = 0u32;
    let mut pow = 0u32;
    for (index, line) in lines.enumerate() {
        let draws: Vec<_> = line
            .split(':')
            .nth(1)
            .unwrap()
            .split(';')
            .map(|s| Draw::from_str(s).unwrap())
            .collect();

        if playable(&draws) {
            sum += index as u32 + 1
        }

        pow += power(&draws);
    }

    (sum, pow)
}

fn main() {
    println!("test  {:?}", solution("day02/input/test.txt"));
    println!("input {:?}", solution("day02/input/input.txt"));
}
