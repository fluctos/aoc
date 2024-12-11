use std::collections::{HashMap, VecDeque};

fn get_reachable_hills(area_map: &HashMap<(isize, isize), u8>, pos: (isize, isize)) -> HashMap<(isize, isize), u64> {
    let mut candidates: VecDeque<((isize, isize), u8)> = VecDeque::new();
    let mut hills: HashMap<(isize, isize), u64> = HashMap::new();

    candidates.push_back((pos, *area_map.get(&pos).unwrap()));
    while let Some((pos, level)) = candidates.pop_front() {

        if level == 9 {
            *hills.entry(pos).or_insert(0) += 1;
            continue;
        }

        let neighbors: [(isize, isize); 4] = [
            (pos.0,     pos.1 + 1),
            (pos.0 + 1, pos.1    ),
            (pos.0,     pos.1 - 1),
            (pos.0 - 1, pos.1    )
        ];

        for neighbor in neighbors {
            if let Some(&new_level) = area_map.get(&neighbor) {
                if new_level == level + 1 {
                    candidates.push_back((neighbor, new_level));
                }
            }
        }
    }

    hills
}

fn solve(data: &str) -> (u64, u64) {
    let mut area_map: HashMap<(isize, isize), u8> = HashMap::new();
    let mut lowest: Vec<(isize, isize)> = Vec::new();

    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let (y, x) = (row as isize, col as isize);

            area_map.insert((y, x), c.to_digit(10).unwrap() as u8);

            if c == '0' {
                lowest.push((y, x));
            }
        }
    }

    lowest
        .into_iter()
        .map(|pos| get_reachable_hills(&area_map, pos))
        .fold((0_u64, 0_u64), |acc, e| {
            (acc.0 + e.keys().len() as u64, acc.1 + e.values().sum::<u64>())
        })
}

fn solve_part_one(data: &str) -> u64 {
    solve(data).0
}


fn solve_part_two(data: &str) -> u64 {
    solve(data).1
}

fn main() {
    let data = std::fs::read_to_string("day10/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "89010123\n\
                         78121874\n\
                         87430965\n\
                         96549874\n\
                         45678903\n\
                         32019012\n\
                         01329801\n\
                         10456732";

    #[test]
    fn day10_part_one() {
        assert_eq!(solve_part_one(INPUT), 36);
    }

    #[test]
    fn day10_part_two() {
        assert_eq!(solve_part_two(INPUT), 81);
    }
}
