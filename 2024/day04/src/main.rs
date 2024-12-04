use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos(i64, i64);

struct CharMap {
    x: HashSet<Pos>,
    m: HashSet<Pos>,
    a: HashSet<Pos>,
    s: HashSet<Pos>,
}

impl CharMap {
    fn new() -> Self {
        CharMap {
            x: HashSet::new(),
            m: HashSet::new(),
            a: HashSet::new(),
            s: HashSet::new(),
        }
    }
}

impl FromStr for CharMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_map = CharMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                match char {
                    'X' => { char_map.x.insert(Pos(row as i64, col as i64)); }
                    'M' => { char_map.m.insert(Pos(row as i64, col as i64)); }
                    'A' => { char_map.a.insert(Pos(row as i64, col as i64)); }
                    'S' => { char_map.s.insert(Pos(row as i64, col as i64)); }
                    _ => unreachable!(),
                }
            }
        }

        Ok(char_map)
    }
}

const N: i64 = -1;
const W: i64 = -1;
const S: i64 =  1;
const E: i64 =  1;

fn solve_part_one(char_map: &CharMap) -> u64 {
    let mut num_words = 0u64;

    let options: [(i64, i64); 8] = [
        (0, E), (S, E), (S, 0), (S, W), (0, W), (N, W), (N, 0), (N, E)
    ];

    for Pos(xrow, xcol) in char_map.x.iter() {
        for (row, col) in options {
            let maybe_found = [
                char_map.m.get(&Pos(xrow + 1 * row, xcol + 1 * col)),
                char_map.a.get(&Pos(xrow + 2 * row, xcol + 2 * col)),
                char_map.s.get(&Pos(xrow + 3 * row, xcol + 3 * col)),
            ];

            if maybe_found.iter().all(|&opt| opt.is_some()) {
                num_words += 1;
            }
        }
    }

    num_words
}

fn solve_part_two(char_map: &CharMap) -> u64 {
    let mut num_words = 0u64;

    let options: [[(i64, i64); 2]; 4] = [
        [(S, E), (S, W)],
        [(N, W), (S, W)],
        [(N, W), (N, E)],
        [(S, E), (N, E)],
    ];

    for Pos(arow, acol) in char_map.a.iter() {
        for [(m1row, m1col), (m2row, m2col)] in options {
            let maybe_found = [
                char_map.m.get(&Pos(arow + m1row, acol + m1col)),
                char_map.m.get(&Pos(arow + m2row, acol + m2col)),
                char_map.s.get(&Pos(arow - m1row, acol - m1col)),
                char_map.s.get(&Pos(arow - m2row, acol - m2col)),
            ];

            if maybe_found.iter().all(|&opt| opt.is_some()) {
                num_words += 1;
            }
        }
    }

    num_words
}

fn main() {
    let data = std::fs::read_to_string("day04/input.txt").unwrap();
    let char_map = CharMap::from_str(data.as_str()).expect("AOC inputs are assumed to be well-formed");

    println!("Part one: {}", solve_part_one(&char_map));
    println!("Part two: {}", solve_part_two(&char_map));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "MMMSXXMASM\n\
                         MSAMXMSMSA\n\
                         AMXSXMAAMM\n\
                         MSAMASMSMX\n\
                         XMASAMXAMM\n\
                         XXAMMXXAMA\n\
                         SMSMSASXSS\n\
                         SAXAMASAAA\n\
                         MAMMMXMMMM\n\
                         MXMXAXMASX";
    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(&CharMap::from_str(INPUT).unwrap()), 18);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part_two(&CharMap::from_str(INPUT).unwrap()), 9);
    }

}
