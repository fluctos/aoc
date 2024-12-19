use std::str::FromStr;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Robot {
    pos: (u64, u64),
    vel: (i64, i64),
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .split([',', ' '])
            .map(|s| s.trim_start_matches("p="))
            .map(|s| s.trim_start_matches("v="))
            .map(|v| v.parse::<i64>().unwrap());

        let (x, y, vx, vy) = (
            iter.next().unwrap() as u64,
            iter.next().unwrap() as u64,
            iter.next().unwrap(),
            iter.next().unwrap()
        );

        Ok(Self{
            pos: ( y,  x),
            vel: (vy, vx),
        })
    }
}

impl Robot {
    fn pos_after(&self, secs: u64, limits: (u64, u64)) -> Self {
        let wrap = |pos, limit| -> u64 {
            let new_pos = match pos % limit as i64 {
                p if p <  0 => p + limit as i64,
                p if p >= 0 => p,
                _ => unreachable!(),
            };

            new_pos as u64
        };

        let new_y = wrap(self.pos.0 as i64 + self.vel.0 * secs as i64, limits.0);
        let new_x = wrap(self.pos.1 as i64 + self.vel.1 * secs as i64, limits.1);

        Robot {pos: (new_y, new_x), ..*self}
    }
}

fn to_quadrant(pos: (u64, u64), limits: (u64, u64)) -> Option<usize> {
    let (y, x) = pos;
    let (height, width) = limits;

    if y == height / 2 || x == width / 2 {
        return None;
    }

    Some(2 * (y * 2 / height) as usize + (x * 2 / width) as usize)
}

fn solve_part_one(data: &str, limits: (u64, u64)) -> u64 {
    let robots: Vec<Robot> = data.lines()
        .map(|line| Robot::from_str(line).unwrap())
        .collect();

    let mut quadrants: [u64; 4] = [0, 0, 0, 0];

    robots .iter()
        .map(|r| r.pos_after(100, limits))
        .filter_map(|robot| to_quadrant(robot.pos, limits))
        .for_each(|q| quadrants[q] += 1);

    quadrants
        .iter()
        .product()
}

fn visualise(robots: &Vec<Robot>, limits: (u64, u64)) {
    let mut positions: HashSet<(u64, u64)> = HashSet::new();
    for robot in robots {
        positions.insert(robot.pos);
    }

    for row in 0..limits.0 {
        for col in 0..limits.1 {
            print!("{}", {
                if positions.contains(&(row, col)) {
                    '#'
                } else {
                    '.'
                }
            });
        }
        println!();
    }
}

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut x0, mut x1) = (1, 0);
    let (mut y0, mut y1) = (0, 1);

    let (mut a, mut b) = (a, b);

    while b != 0 {
        let quotient = a / b;
        let remainder = a % b;

        (a, b) = (b, remainder);

        (x0, x1) = (x1, x0 - quotient * x1);
        (y0, y1) = (y1, y0 - quotient * y1);
    }

    (a, x0, y0)
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b).0
}

fn solve_part_two(data: &str, limits: (u64, u64)) -> u64 {
    let robots: Vec<Robot> = data.lines()
        .map(|line| Robot::from_str(line).unwrap())
        .collect();

    let max_steps = robots
        .iter()
        .map(|r| {
            let nx = limits.0 as i64 / gcd(limits.0 as i64, r.vel.0).0;
            let ny = limits.1 as i64 / gcd(limits.1 as i64, r.vel.1).0;

            lcm(nx, ny)
        })
        .reduce(|acc, elem| lcm(acc, elem))
        .unwrap() as u64;

    let mut min_metric = i64::MAX;
    let mut min_elapsed = 0;

    for s in 1..=max_steps {
        let tmp_robots: Vec<_> = robots
            .iter()
            .map(|r| r.pos_after(s, limits))
            .collect();

        let spread_metric: i64 = tmp_robots
            .iter()
            .map(|r| i64::abs(limits.0 as i64 / 2 - r.pos.0 as i64) + i64::abs(limits.1 as i64 / 2 - r.pos.1 as i64))
            .sum();

        if spread_metric < min_metric {
            min_metric = spread_metric;
            min_elapsed = s;
            println!("\nElapsed: {s}, min_metric: {min_metric} -> ");
            visualise(&tmp_robots, limits);
        }
    }

    min_elapsed
}

fn main() {
    let data = std::fs::read_to_string("day14/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str(), (103, 101)));
    println!("Part two: {}", solve_part_two(data.as_str(), (103, 101)));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3\n\
                         p=6,3 v=-1,-3\n\
                         p=10,3 v=-1,2\n\
                         p=2,0 v=2,-1\n\
                         p=0,0 v=1,3\n\
                         p=3,0 v=-2,-2\n\
                         p=7,6 v=-1,-3\n\
                         p=3,0 v=-1,-2\n\
                         p=9,3 v=2,3\n\
                         p=7,3 v=-1,2\n\
                         p=2,4 v=2,-3\n\
                         p=9,5 v=-3,-3";
    #[test]
    fn day14_part_one() {
        assert_eq!(solve_part_one(INPUT, (7, 11)), 12);
    }

    #[test]
    fn day14_part_two() {
        assert_eq!(solve_part_two(INPUT, (7, 11)), 0);
    }

    #[test]
    fn day14_modulo_test() {
        assert_eq!(-10 % 3, -1);
    }

    #[test]
    fn day14_movement_test() {
        let limits = (3_u64, 5_u64);
        let robot = Robot{pos: (1, 1), vel: (-1, -1)};
        assert_eq!(robot.pos_after(0, limits), robot);
        assert_eq!(robot.pos_after(1, limits), Robot{pos: (0, 0), ..robot});
        assert_eq!(robot.pos_after(2, limits), Robot{pos: (2, 4), ..robot});
        assert_eq!(robot.pos_after(3, limits), Robot{pos: (1, 3), ..robot});
    }
}
