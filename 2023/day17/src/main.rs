use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    N, E, S, W
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Vector {
    position: (isize, isize),
    direction: Direction,
}

impl Vector {
    fn new(position: (isize, isize), direction: Direction) -> Self {
        Self{position, direction}
    }

    fn turn_cw(&self) -> Self {
        let new_direction = match self.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };

        Self::new(self.position, new_direction)
    }

    fn turn_ccw(&self) -> Self {
        let new_direction = match self.direction {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        };

        Self::new(self.position, new_direction)
    }

    fn step(&self) -> Self {
        let new_position = match self.direction {
            Direction::N => (self.position.0 - 1, self.position.1),
            Direction::E => (self.position.0,     self.position.1 + 1),
            Direction::S => (self.position.0 + 1, self.position.1),
            Direction::W => (self.position.0,     self.position.1 - 1),
        };

        Self::new(new_position, self.direction)
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Tracer {
    vector: Vector,
    steps_forward: usize,
}

impl Tracer {
    fn new(position: (isize, isize), direction: Direction) -> Self {
        Self{
            vector: Vector::new(position, direction),
            steps_forward: 0,
        }
    }

    fn forward(&self) -> Self {
        Self {
            vector: self.vector.step(),
            steps_forward: self.steps_forward + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            vector: self.vector.turn_ccw().step(),
            steps_forward: 1
        }
    }

    fn right(&self) -> Self {
        Self {
            vector: self.vector.turn_cw().step(),
            steps_forward: 1,
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    tracer: Tracer,
    cost: u64,
}

impl State {
    fn new(tracer: Tracer, cost: u64) -> Self {
        Self{tracer, cost}
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
             .then_with(|| self.tracer.vector.position.cmp(&other.tracer.vector.position))
    }
}

fn get_val<T: Into<u64> + Copy>(grid: &Vec<Vec<T>>, pos: &(isize, isize)) -> Option<u64> {
    if pos.0 < 0 || pos.0 as usize >= grid.len() || pos.1 < 0 || pos.1 as usize >= grid[0].len() {
        None
    } else {
        Some(grid[pos.0 as usize][pos.1 as usize].into())
    }
}

fn solve(grid: &Vec<Vec<u8>>, adjacency_fn: &dyn Fn(Tracer) -> Vec<Tracer>, end_condition: &dyn Fn(Tracer) -> bool) -> u64 {
    let mut candidates = BinaryHeap::new();
    let mut distances: HashMap<Tracer, u64> = HashMap::new();

    let init: Vec<Tracer> = vec![Tracer::new((0, 0), Direction::E), Tracer::new((0, 0), Direction::S)];

    for tracer in init {
        candidates.push(State::new(tracer, 0));
        distances.insert(tracer, 0);
    }

    while let Some(State{tracer, cost}) = candidates.pop() {
        if end_condition(tracer) {
           return cost;
        }

        let current_cost = *distances.get(&tracer).unwrap_or(&u64::max_value());
        if cost > current_cost {
            continue
        }

        let neighbors = adjacency_fn(tracer);

        for neighbor in neighbors {
            let neighbor_cost = get_val(grid, &neighbor.vector.position).unwrap();
            let next_state = State::new(neighbor, cost + neighbor_cost);
            if next_state.cost < *distances.get(&next_state.tracer).unwrap_or(&u64::max_value()) {
                *distances.entry(next_state.tracer).or_insert(u64::max_value()) = next_state.cost;
                candidates.push(next_state);
            }
        }
    }

    u64::max_value()
}

fn filter_outside_grid(mut tracers: Vec<Tracer>, grid: &Vec<Vec<u8>>) -> Vec<Tracer> {
    tracers.retain(|&tracer| tracer.vector.position.0 >= 0 && (tracer.vector.position.0 as usize) < grid.len() &&
                             tracer.vector.position.1 >= 0 && (tracer.vector.position.1 as usize) < grid[0].len());
    tracers
}

fn solution(input_file_path: &str) -> (u64, u64) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let grid: Vec<Vec<u8>> = data.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect();

    let part_1_adjacency = |tracer: Tracer| -> Vec<Tracer> {
        let neighbors = if tracer.steps_forward < 3 {
            vec![tracer.left(), tracer.right(), tracer.forward()]
        } else {
            vec![tracer.left(), tracer.right()]
        };

        filter_outside_grid(neighbors, &grid)
    };

    let part_1_end = |tracer: Tracer| -> bool {
        tracer.vector.position.0 as usize == grid.len() - 1 &&
        tracer.vector.position.1 as usize == grid[0].len() - 1
    };

    let part_2_adjacency = |tracer: Tracer| -> Vec<Tracer> {
        let neighbors = if tracer.steps_forward < 4 {
            vec![tracer.forward()]
        } else if tracer.steps_forward >= 4 && tracer.steps_forward < 10 {
            vec![tracer.left(), tracer.right(), tracer.forward()]
        } else {
            vec![tracer.left(), tracer.right()]
        };

        filter_outside_grid(neighbors, &grid)
    };

    let part_2_end = |tracer: Tracer| -> bool {
        tracer.vector.position.0 as usize == grid.len() - 1 &&
        tracer.vector.position.1 as usize == grid[0].len() - 1 &&
        tracer.steps_forward >= 4
    };

    let part_1 = solve(&grid, &part_1_adjacency, &part_1_end);
    let part_2 = solve(&grid, &part_2_adjacency, &part_2_end);

    (part_1, part_2)
}

fn main() {
    println!("{:?}", solution("day17/input/test.txt"));
    println!("{:?}", solution("day17/input/test_2.txt"));
    println!("{:?}", solution("day17/input/input.txt"));
}

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn test_tracer_movement() {
        let tracer = Tracer::new((0, 0), Direction::E).right().right().right().right();

        assert_eq!(tracer.vector.position, (0, 0));
        assert_eq!(tracer.vector.direction, Direction::E);
        assert_eq!(tracer.steps_forward, 1);

        let tracer = Tracer::new((0, 0), Direction::N).left().left().left().left();

        assert_eq!(tracer.vector.position, (0, 0));
        assert_eq!(tracer.vector.direction, Direction::N);
        assert_eq!(tracer.steps_forward, 1);

        let tracer = Tracer::new((0, 0), Direction::N).right().forward().forward().forward();

        assert_eq!(tracer.vector.position, (0, 4));
        assert_eq!(tracer.vector.direction, Direction::E);
        assert_eq!(tracer.steps_forward, 4);
    }
}
