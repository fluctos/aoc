use std::str::FromStr;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos(i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Guard {
    pos: Pos,
    dir: Direction,
}

impl Guard {
    fn new(pos: Pos, dir: Direction) -> Self {
        Self{pos, dir}
    }

    fn step(&self) -> Guard {
        match self.dir {
            Direction::Up    => Guard::new(Pos(self.pos.0 - 1, self.pos.1),     self.dir),
            Direction::Right => Guard::new(Pos(self.pos.0,     self.pos.1 + 1), self.dir),
            Direction::Down  => Guard::new(Pos(self.pos.0 + 1, self.pos.1),     self.dir),
            Direction::Left  => Guard::new(Pos(self.pos.0,     self.pos.1 - 1), self.dir),
        }
    }

    fn rotate_cw(&self) -> Guard {
        match self.dir {
            Direction::Up    => Guard::new(self.pos, Direction::Right),
            Direction::Right => Guard::new(self.pos, Direction::Down),
            Direction::Down  => Guard::new(self.pos, Direction::Left),
            Direction::Left  => Guard::new(self.pos, Direction::Up),
        }
    }
}

struct Grid {
    data: Vec<Vec<char>>
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let grid = Grid{
            data: str
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect()
        };

        Ok(grid)
    }
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> Option<char> {
        if row < self.data.len() && col < self.data[0].len() {
            return Some(self.data[row][col])
        }

        None
    }

    fn get_by_pos(&self, pos: &Pos) -> Option<char> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }

        self.get(pos.0 as usize, pos.1 as usize)
    }

    fn set(&mut self, row: usize, col: usize, chr: char) {
        if row < self.data.len() && col < self.data[0].len() {
            self.data[row][col] = chr;
        }
    }
}

fn get_start_pos(grid: &Grid) -> Option<Pos> {
    for (y, row) in grid.data.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                return Some(Pos(y as i64, x as i64));
            }
        }
    }
    None
}

enum Outcome {
    OutOfGrid(HashSet<Pos>),
    Loop(),
}

fn next_step(guard: Guard, grid: &Grid) -> Option<Guard> {
    match grid.get_by_pos(&guard.step().pos) {
        None => None,
        Some('#') => next_step(guard.rotate_cw(), grid),
        Some(_) => Some(guard.step()),
    }
}

fn distinct_pos(guards: HashSet<Guard>) -> HashSet<Pos> {
    guards
        .into_iter()
        .map(|g| g.pos)
        .collect()
}

fn simulate(grid: &Grid, guard: &Guard) -> Outcome {
    let mut guard = guard.clone();
    let mut visited = HashSet::new();

    visited.insert(guard);

    while let Some(next) = next_step(guard, &grid) {
        if visited.contains(&next) {
            return Outcome::Loop();
        }
        guard = next.clone();
        visited.insert(next);
    }

    Outcome::OutOfGrid(distinct_pos(visited))
}


fn solve_part_one(data: &str) -> u64 {
    let grid = Grid::from_str(data).unwrap();
    let guard = Guard::new(get_start_pos(&grid).unwrap(), Direction::Up);
    match simulate(&grid, &guard) {
        Outcome::OutOfGrid(positions) => positions.len() as u64,
        Outcome::Loop() => unreachable!(),
    }
}

fn solve_part_two(data: &str) -> u64 {
    let mut grid = Grid::from_str(data).unwrap();
    let start_pos = get_start_pos(&grid).unwrap();
    let guard = Guard::new(start_pos, Direction::Up);
    let mut num_loops = 0u64;

    let orig_route = match simulate(&grid, &guard) {
        Outcome::OutOfGrid(visited) => visited,
        Outcome::Loop() => unreachable!(),
    };

    for pos in orig_route.into_iter().filter(|&p| p != start_pos) {
        let c = grid.get_by_pos(&pos).unwrap();

        grid.set(pos.0 as usize, pos.1 as usize, '#');

        match simulate(&grid, &guard) {
            Outcome::Loop() => num_loops += 1,
            _ => (),
        }

        grid.set(pos.0 as usize, pos.1 as usize, c);
    }

    num_loops
}

fn main() {
    let data = std::fs::read_to_string("day06/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "....#.....\n\
                         .........#\n\
                         ..........\n\
                         ..#.......\n\
                         .......#..\n\
                         ..........\n\
                         .#..^.....\n\
                         ........#.\n\
                         #.........\n\
                         ......#...";

    #[test]
    fn day06_part_one() {
        assert_eq!(solve_part_one(INPUT), 41);
    }

    #[test]
    fn day06_part_two() {
        assert_eq!(solve_part_two(INPUT), 6);
    }
}
