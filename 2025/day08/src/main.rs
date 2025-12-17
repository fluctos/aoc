use std::error::Error;
use std::process::ExitCode;

type Point = (u64, u64, u64);

fn get_points(data: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    data.lines()
        .map(|line| {
            let mut tokens = line.split(',');
            let x = tokens.next().ok_or("Malformed input")?;
            let y = tokens.next().ok_or("Malformed input")?;
            let z = tokens.next().ok_or("Malformed input")?;
            let (x, y, z) = (x.parse()?, y.parse()?, z.parse()?);
            Ok((x, y, z))
        })
        .collect()
}

fn get_distances(points: &[Point]) -> Vec<(u128, usize, usize)> {
    let n = points.len();
    let mut distances: Vec<(u128, usize, usize)> = Vec::with_capacity(n*(n-1)/2);

    for u_idx in 0..n.saturating_sub(1) {
        for v_idx in (u_idx + 1)..points.len() {
            let (ux, uy, uz) = points[u_idx];
            let (vx, vy, vz) = points[v_idx];

            let dx = ux.abs_diff(vx) as u128;
            let dy = uy.abs_diff(vy) as u128;
            let dz = uz.abs_diff(vz) as u128;

            let distance = dx*dx + dy*dy + dz*dz;

            distances.push((distance, u_idx, v_idx));
        }
    }

    distances
}

struct DisjointSetUnion {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    num_sets: usize,
}

impl DisjointSetUnion {
    fn new(size: usize) -> Self {
        DisjointSetUnion {
            parents: (0..size).collect(),
            sizes: vec![1; size],
            num_sets: size,
        }
    }

    fn find_root(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find_root(self.parents[x]);
        }
        self.parents[x]
    }

    fn merge(&mut self, a: usize, b: usize) {
        let mut root_a = self.find_root(a);
        let mut root_b = self.find_root(b);

        if root_a == root_b {
            return;
        }

        if self.sizes[root_a] < self.sizes[root_b] {
            (root_a, root_b) = (root_b, root_a)
        }

        self.parents[root_b] = root_a;
        self.sizes[root_a] += self.sizes[root_b];
        self.num_sets -= 1;
    }

    fn get_three_largest_sets(&self) -> (usize, usize, usize) {
        let mut largest = (0usize, 0usize, 0usize);

        assert_eq!(self.sizes.len(), self.parents.len());

        for idx in 0..self.sizes.len() {
            if self.parents[idx] != idx {
                continue;
            }

            let s = &self.sizes[idx];

            largest = match (*s > largest.0, *s > largest.1, *s > largest.2) {
                (true,  true,  true) => (*s, largest.0, largest.1),
                (false, true,  true) => (largest.0, *s, largest.1),
                (false, false, true) => (largest.0, largest.1, *s),
                _ => (largest.0, largest.1, largest.2),
            }
        }

        largest
    }

    fn get_num_sets(&self) -> usize {
        self.num_sets
    }
}

fn three_largest(data: &str, num_iters: usize) -> Result<u64, Box<dyn Error>> {
    let points = get_points(data)?;
    let mut distances = get_distances(&points);

    distances.sort_unstable_by_key(|elem| elem.0);

    let mut dsu = DisjointSetUnion::new(points.len());

    let num_iters = usize::min(num_iters, distances.len());
    for idx in 0..num_iters {
        let (_, a, b) = distances[idx];
        dsu.merge(a, b);
    }

    let largest = dsu.get_three_largest_sets();

    Ok(largest.0 as u64 * largest.1 as u64 * largest.2 as u64)
}

fn single_set(data: &str) -> Result<u64, Box<dyn Error>> {
    let points = get_points(data)?;
    let mut distances = get_distances(&points);

    distances.sort_unstable_by_key(|elem| elem.0);

    let mut dsu = DisjointSetUnion::new(points.len());

    for idx in 0..distances.len() {
        let (_, a, b) = distances[idx];

        dsu.merge(a, b);

        if dsu.get_num_sets() == 1 {
            return Ok(points[a].0 as u64 * points[b].0 as u64);
        }
    }

    unreachable!();
}

fn solve_part_one(data: &str) -> Result<u64, Box<dyn Error>> {
    three_largest(data, 1000)
}

fn solve_part_two(data: &str) -> Result<u64, Box<dyn Error>> {
    single_set(data)
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("day08/input.txt")?;
    let answer_one = solve_part_one(&data)?;
    let answer_two = solve_part_two(&data)?;

    println!("Part one: {}", answer_one);
    println!("Part two: {}", answer_two);

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = concat!(
        "162,817,812\n",
        "57,618,57\n",
        "906,360,560\n",
        "592,479,940\n",
        "352,342,300\n",
        "466,668,158\n",
        "542,29,236\n",
        "431,825,988\n",
        "739,650,466\n",
        "52,470,668\n",
        "216,146,977\n",
        "819,987,18\n",
        "117,168,530\n",
        "805,96,715\n",
        "346,949,466\n",
        "970,615,88\n",
        "941,993,340\n",
        "862,61,35\n",
        "984,92,344\n",
        "425,690,689"
    );

    #[test]
    fn day08_part_one() {
        assert_eq!(three_largest(TEST_INPUT, 10).unwrap(), 40);
    }

    #[test]
    fn day08_part_two() {
        assert_eq!(single_set(TEST_INPUT).unwrap(), 25272);
    }
}
