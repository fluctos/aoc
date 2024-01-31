use std::cmp::max;

enum Heading {
    N, E, S, W
}

struct TracerState {
    y: isize,
    x: isize,
    heading: Heading,
}

fn trace(grid: &mut Vec<Vec<char>>, y: usize, x: usize, heading: Heading) -> usize {
    let mut tracers: Vec<TracerState> = Vec::new();
    let mut visited: Vec<Vec<char>> = (0..grid.len()).map(|_| {std::iter::repeat('.').take(grid[0].len()).collect()}).collect();

    tracers.push(TracerState{y: y as isize, x: x as isize, heading});

    while !tracers.is_empty() {
        match (grid[tracers[0].y as usize][tracers[0].x as usize], &tracers[0].heading) {
            ('\\', Heading::N) => tracers[0].heading = Heading::W,
            ('\\', Heading::E) => tracers[0].heading = Heading::S,
            ('\\', Heading::S) => tracers[0].heading = Heading::E,
            ('\\', Heading::W) => tracers[0].heading = Heading::N,

            ('/', Heading::N) => tracers[0].heading = Heading::E,
            ('/', Heading::E) => tracers[0].heading = Heading::N,
            ('/', Heading::S) => tracers[0].heading = Heading::W,
            ('/', Heading::W) => tracers[0].heading = Heading::S,

            ('-', Heading::N | Heading::S) => {
                tracers.push(TracerState{x: tracers[0].x, y: tracers[0].y, heading: Heading::E});
                tracers[0].heading = Heading::W;
            },

            ('|', Heading::E | Heading::W) => {
                tracers.push(TracerState{x: tracers[0].x, y: tracers[0].y, heading: Heading::S});
                tracers[0].heading = Heading::N;
            },

            _ => (),
        }

        tracers.retain(|s| {
            match (&s.heading, visited[s.y as usize][s.x as usize]) {
                (Heading::N, '^') => false,
                (Heading::E, '>') => false,
                (Heading::S, 'v') => false,
                (Heading::W, '<') => false,
                _ => true,
            }
        });

        if tracers.is_empty() {
            break;
        }

        visited[tracers[0].y as usize][tracers[0].x as usize] = match tracers[0].heading {
            Heading::N => '^',
            Heading::E => '>',
            Heading::S => 'v',
            Heading::W => '<',
        };

        match tracers[0].heading {
            Heading::N => tracers[0].y -= 1,
            Heading::E => tracers[0].x += 1,
            Heading::S => tracers[0].y += 1,
            Heading::W => tracers[0].x -= 1,
        }

        tracers.retain(|s| {
            s.y >= 0 &&
            s.y < grid.len() as isize &&
            s.x >= 0 &&
            s.x < grid[0].len() as isize
        });
    }

    visited.into_iter().flatten().filter(|&c| c != '.').count()
}

fn solution(input_file_path: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    assert!(grid.len() == grid[0].len());
    let size = grid.len();
    let part_a = trace(&mut grid, 0, 0, Heading::E);
    let part_b = max(
        (0..size).map(|row| max(trace(&mut grid, row, 0, Heading::E), trace(&mut grid, row, size - 1, Heading::W))).max().unwrap(),
        (0..size).map(|col| max(trace(&mut grid, 0, col, Heading::S), trace(&mut grid, size - 1, col, Heading::N))).max().unwrap()
    );

    (part_a, part_b)
}

fn main() {
    println!("{:?}", solution("day16/input/test.txt"));
    println!("{:?}", solution("day16/input/input.txt"));
}
