use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
    rows: i64,
    cols: i64,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let rows: i64 = data.len() as i64;
        let cols: i64 = data[0].len() as i64;

        Ok(Grid{data, rows, cols })
    }
}

impl Grid {
    fn get_s_pos(&self) -> Option<(i64, i64)> {
        for (y, row) in self.data.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                if *char == 'S' {
                    return Some((y as i64, x as i64))
                }
            }
        }
        None
    }

    fn wrap(coord: i64, limit: i64) -> i64 {
        ((coord % limit) + limit) % limit
    }

    fn get_neighbors(&self, point: (i64, i64), wrap_around: bool) -> impl Iterator<Item = (i64, i64)> + '_ {
        let (y, x) = point;

        let neighbors = [
            (y - 1, x),
            (y, x - 1),
            (y + 1, x),
            (y, x + 1),
        ];

        neighbors.into_iter().filter_map(move |(y, x)| {
            let condition;
            if wrap_around {
                let y_grid = Self::wrap(y, self.rows) as usize;
                let x_grid = Self::wrap(x, self.cols) as usize;
                condition = self.data[y_grid][x_grid] != '#';
            } else {
                condition = y >= 0 && y < self.rows && x >= 0 && x < self.cols && self.data[y as usize][x as usize] != '#';
            }

            match condition {
                true => Some((y, x)),
                false => None,
            }
        })
    }
}

fn get_candidates(positions: HashSet<(i64, i64)>, grid: &Grid, wrap_around: bool) -> HashSet<(i64, i64)> {
    let mut next = HashSet::new();
    for pos in positions {
        next.extend(grid.get_neighbors(pos, wrap_around));
    }
    next
}

fn calc_positions(grid: &Grid, steps: u64, wrap_around: bool) -> (u64, HashMap<(i64, i64), u64>) {
    let mut positions = HashMap::<(i64, i64), u64>::new();
    let mut candidates = HashSet::<(i64, i64)>::new();

    let mut even_positions = 0u64;
    let mut odd_positions = 0u64;

    candidates.insert(grid.get_s_pos().unwrap());

    let mut duration = Instant::now();

    for iter in 1..=steps {
        candidates = get_candidates(candidates, &grid, wrap_around)
            .into_iter()
            .filter(|c| !positions.contains_key(c))
            .collect();

        if iter % 2 == 0 {
            even_positions += candidates.len() as u64;
        } else {
            odd_positions += candidates.len() as u64;
        }

        for candidate in candidates.iter() {
            positions.entry(candidate.clone()).or_insert(iter);
        }

        if iter % 1000 == 0 {
            println!("... Iter: {} Candidates: {} Duration: {:?} Avg: {:?}",
                iter,
                candidates.len(),
                duration.elapsed(),
                duration.elapsed() / candidates.len() as u32);
            duration = Instant::now();
        }
    }

    match steps % 2 {
        0 => (even_positions, positions),
        1 => (odd_positions, positions),
        _ => unreachable!(),
    }
}

fn solve_part_1() {
    let cases = Vec::from([
        ("day21/input/test.txt", vec![
            (6u64, false),
            (6u64, true),
            (10u64, true),
            (50u64, true),
            (100u64, true),
            (500u64, true),
            (1000u64, true),
            (5000u64, true),
        ]),

        ("day21/input/input.txt", vec![
            (64u64, false),
        ]),
    ]);

    for (path, params) in cases {
        let data = std::fs::read_to_string(path).unwrap();
        let grid = Grid::from_str(data.as_str()).unwrap();
        for &(steps, wrap_around) in params.iter() {
            println!("Case: {:?}", (path, steps, wrap_around));
            println!("... Result: {}", calc_positions(&grid, steps, wrap_around).0);
        }
    }
}

fn get_positions(positions: &HashMap<(i64, i64), u64>, y_lim: (i64, i64), x_lim: (i64, i64)) -> (u64, u64) {
    assert!(y_lim.0 <= y_lim.1 && x_lim.0 <= x_lim.1);
    let mut num_odd = 0u64;
    let mut num_even = 0u64;
    for (&(y, x), &step) in positions {
        let proper_y = y >= y_lim.0 && y < y_lim.1;
        let proper_x = x >= x_lim.0 && x < x_lim.1;
        match (proper_y, proper_x, step % 2) {
            (true, true, 0) => num_even += 1,
            (true, true, 1) => num_odd += 1,
            _ => (),
        }
    }

    (num_even, num_odd)
}

fn solve_part_2() {
    let path = "day21/input/input.txt";
    let data = std::fs::read_to_string(&path).unwrap();
    let grid = Grid::from_str(data.as_str()).unwrap();
    let steps = 26501365u64;
    let grid_size = 131u64;

    let mut positions = 0u64;

    assert!(grid.data.len() == grid_size as usize);
    assert!(grid.data.len() == grid.data[0].len());

    let s_pos = grid.get_s_pos().unwrap();
    let s_row_empty = grid.data[s_pos.0 as usize].iter().all(|&c| c != '#');
    let s_col_empty = grid.data.iter().map(|v| v[s_pos.1 as usize]).all(|c| c != '#');

    assert!(s_row_empty && s_col_empty);

    let grids_to_edge = (steps - grid_size / 2) / grid_size;

    assert!(grids_to_edge == 202300);

    let full_grids_to_edge = grids_to_edge - 1;

    assert!(full_grids_to_edge % 2 == 1);

    let full_odd_grids = full_grids_to_edge.pow(2);
    let full_even_grids = (full_grids_to_edge + 1).pow(2);

    let edge_length = grids_to_edge + 1;

    let (_, reference) = calc_positions(&grid, 65 + grid_size * 2, true);

    let tile = |pos:i64| {
        let min = pos * grid_size as i64;
        (min, min + grid_size as i64)
    };

    // Inside
    positions += get_positions(&reference, tile(0), tile(0)).1 * full_odd_grids;
    positions += get_positions(&reference, tile(1), tile(0)).1 * full_even_grids;

    // Corners
    positions += get_positions(&reference, tile(2), tile(0)).1;
    positions += get_positions(&reference, tile(-2), tile(0)).1;
    positions += get_positions(&reference, tile(0), tile(2)).1;
    positions += get_positions(&reference, tile(0), tile(-2)).1;

    // NE edge
    positions += get_positions(&reference, tile(1), tile(1)).1 * (edge_length - 2);
    positions += get_positions(&reference, tile(1), tile(2)).1 * (edge_length - 1);

    // SE edge
    positions += get_positions(&reference, tile(-1), tile(1)).1 * (edge_length - 2);
    positions += get_positions(&reference, tile(-1), tile(2)).1 * (edge_length - 1);

    // NW edge
    positions += get_positions(&reference, tile(1), tile(-1)).1 * (edge_length - 2);
    positions += get_positions(&reference, tile(1), tile(-2)).1 * (edge_length - 1);

    // SW edge
    positions += get_positions(&reference, tile(-1), tile(-1)).1 * (edge_length - 2);
    positions += get_positions(&reference, tile(-1), tile(-2)).1 * (edge_length - 1);

    println!("Case: {:?}", (path, steps, true));
    println!("... Result: {}", positions);
}

fn main() {
    solve_part_1();
    solve_part_2();
}
