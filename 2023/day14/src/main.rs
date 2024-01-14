use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Slope {
    Ascending,
    Descending,
}

fn tilt_verti(mut grid: Vec<Vec<char>>, slope: Slope) -> Vec<Vec<char>> {
    let size = grid.len();
    let mut next_free = vec![0usize; size];
    for row in 0..size {
        for col in 0..size {
            match grid[row][col] {
                '#' => next_free[col] = row + 1,
                'O' if slope == Slope::Ascending => {
                    if next_free[col] != row {
                        grid[next_free[col]][col] = 'O';
                        grid[row][col] = '.';
                    }
                    next_free[col] += 1;
                },
                '.' if slope == Slope::Descending => {
                    if next_free[col] != row {
                        grid[next_free[col]][col] = '.';
                        grid[row][col] = 'O';
                    }
                    next_free[col] += 1;
                },
                _ => (),
            }
        }
    }

    grid
}

fn tilt_horiz(mut grid: Vec<Vec<char>>, slope: Slope) -> Vec<Vec<char>> {
    for row in grid.iter_mut() {
        let mut next_free = 0usize;
        for col in 0..row.len() {
            match &row[col] {
                &'#' => next_free = col + 1,
                &'O' if slope == Slope::Ascending => {
                    if col != next_free {
                        row[col] = '.';
                        row[next_free] = 'O';
                    }
                    next_free += 1;
                },
                &'.' if slope == Slope::Descending => {
                    if col != next_free {
                        row[col] = 'O';
                        row[next_free] = '.';
                    }
                    next_free += 1;
                },
                _ => (),
            }
        }
    }

    grid
}

enum TiltDir {
    N,
    E,
    S,
    W,
}

fn tilt(grid: Vec<Vec<char>>, dir: TiltDir) -> Vec<Vec<char>> {
    match dir {
        TiltDir::N => tilt_verti(grid, Slope::Ascending),
        TiltDir::S => tilt_verti(grid, Slope::Descending),

        TiltDir::W => tilt_horiz(grid, Slope::Ascending),
        TiltDir::E => tilt_horiz(grid, Slope::Descending),
    }
}

fn get_load(grid: &Vec<Vec<char>>) -> u64 {
    let mut load = 0u64;
    let num_rows = grid.len() as u64;
    for (row, line) in grid.iter().enumerate() {
        for chr in line.iter() {
            match chr {
                'O' => load += num_rows - row as u64,
                _ => (),
            }
        }
    }

    load
}

fn solution(input_file_path: &str) -> (u64, u64) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    assert!(grid.len() > 0);
    assert!(grid.len() == grid[0].len());

    let part_a = get_load(&tilt(grid.clone(), TiltDir::N));

    let mut part_b = 0u64;
    let mut load_cache: HashMap<u64, (u64, usize)> = HashMap::new(); // grid_hash -> (grid_load, prev_cycle)

    let num_cycles = 1_000_000_000;

    for cycle in 1..=num_cycles {
        grid = tilt(grid, TiltDir::N);
        grid = tilt(grid, TiltDir::W);
        grid = tilt(grid, TiltDir::S);
        grid = tilt(grid, TiltDir::E);

        let mut hasher = DefaultHasher::new();
        grid.hash(&mut hasher);
        let hash = hasher.finish();

        if load_cache.contains_key(&hash) {
            let (load, prev_cycle) = load_cache.get(&hash).unwrap();
            let cadence_len = cycle - prev_cycle;
            let remaining_cycles = num_cycles - cycle;
            if remaining_cycles % cadence_len == 0 {
                part_b = *load;
                break;
            }
        }

        load_cache.entry(hash)
                  .and_modify(|(_, prev_cycle)| *prev_cycle = cycle)
                  .or_insert((get_load(&grid), cycle));
    }

    (part_a, part_b)
}

fn main() {
    println!("{:?}", solution("day14/input/test.txt"));
    println!("{:?}", solution("day14/input/input.txt"));
}
