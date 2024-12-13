use std::collections::{HashMap, HashSet};

struct Plot {
    pos: (usize, usize),
    plant_type: char,
    group_id_index: usize,
    num_neighbors: usize,
}

impl Plot {
    fn new(pos: (usize, usize), plant_type: char, group_id_index: usize) -> Self {
        Plot{pos, plant_type, group_id_index, num_neighbors: 0_usize}
    }
}

fn merge_groups(group_ids: &mut Vec<usize>, first_id: usize, second_id: usize) {
    group_ids
        .iter_mut()
        .for_each(|id| {
            if *id == first_id || *id == second_id {
                *id = usize::min(first_id, second_id);
            }
        });
}

fn parse_grid(data: &str) -> HashMap<usize, Vec<Plot>> {
    let mut indices = (0usize..).into_iter();
    let mut grid: Vec<Vec<Plot>> = data
        .lines()
        .enumerate()
        .map(|(row, line)| line
            .chars()
            .enumerate()
            .map(|(col, c)| Plot::new((row, col), c, indices.next().unwrap()))
            .collect()
        ).collect();

    assert!(grid.len() > 1);
    assert!(grid[0].len() > 1);

    let mut group_ids: Vec<usize> = (0_usize..)
        .into_iter()
        .take(grid.len() * grid[0].len())
        .collect();

    // Horizontal adjacency
    for row in 0..grid.len() {
        for col in 0..grid[0].len() - 1 {
            if grid[row][col].plant_type == grid[row][col + 1].plant_type {
                let left_id = group_ids[grid[row][col].group_id_index];
                let right_id = group_ids[grid[row][col + 1].group_id_index];
                merge_groups(&mut group_ids, left_id, right_id);
                grid[row][col].num_neighbors += 1;
                grid[row][col + 1].num_neighbors += 1;
            }
        }
    }

    // Vertical adjacency
    for col in 0..grid[0].len() {
        for row in 0..grid.len() - 1 {
            if grid[row][col].plant_type == grid[row + 1][col].plant_type {
                let upper_id = group_ids[grid[row][col].group_id_index];
                let lower_id = group_ids[grid[row + 1][col].group_id_index];
                merge_groups(&mut group_ids, upper_id, lower_id);
                grid[row][col].num_neighbors += 1;
                grid[row + 1][col].num_neighbors += 1;
            }
        }
    };

    let mut regions: HashMap<usize, Vec<Plot>> = HashMap::new();
    for row in grid.into_iter() {
        for plot in row.into_iter() {
            let group_id = group_ids[plot.group_id_index];
            regions.entry(group_id).or_default().push(plot);
        }
    }

    regions
}

fn solve_part_one(data: &str) -> u64 {
    let regions = parse_grid(data);
    regions
        .values()
        .map(|region| {
            let area = region.len() as u64;
            let perimeter = region.iter().map(|plot| 4 - plot.num_neighbors as u64).sum::<u64>();
            area * perimeter
        })
        .sum()
}

fn calc_discount_price(region: &mut Vec<Plot>) -> u64 {

    let mut plots: HashSet<(i64, i64)> = HashSet::new();

    let mut min_row = i64::MAX;
    let mut max_row = i64::MIN;
    let mut min_col = i64::MAX;
    let mut max_col = i64::MIN;

    for plot in region.iter() {
        let (row, col) = (plot.pos.0 as i64, plot.pos.1 as i64);

        plots.insert((row, col));

        min_row = i64::min(min_row, row);
        min_col = i64::min(min_col, col);
        max_row = i64::max(max_row, row);
        max_col = i64::max(max_col, col);
    }

    #[derive(PartialEq, Eq, Hash)]
    enum EdgeType {
        Entry,
        Exit
    }

    let mut horiz_edge_starts: HashSet<(i64, i64, EdgeType)> = HashSet::new();
    let mut horiz_edge_ends: HashSet<(i64, i64, EdgeType)> = HashSet::new();
    let mut vert_edge_starts: HashSet<(i64, i64, EdgeType)> = HashSet::new();
    let mut vert_edge_ends: HashSet<(i64, i64, EdgeType)> = HashSet::new();

    let mut num_sides = 0_u64;

    // Vert edges
    for row in min_row..=max_row {
        for col in min_col - 1..=max_col {
            match (plots.get(&(row, col)), plots.get(&(row, col + 1))) {
                (None, Some(_)) => {
                    let (edge_start, edge_end) = ((row, col + 1, EdgeType::Entry), (row + 1, col + 1, EdgeType::Entry));
                    if let (None, None) = (vert_edge_starts.get(&edge_end), vert_edge_ends.get(&edge_start)) {
                        num_sides += 1;
                    }
                    vert_edge_starts.insert(edge_start);
                    vert_edge_ends.insert(edge_end);
                },
                (Some(_), None) => {
                    let (edge_start, edge_end) = ((row, col + 1, EdgeType::Exit), (row + 1, col + 1, EdgeType::Exit));
                    if let (None, None) = (vert_edge_starts.get(&edge_end), vert_edge_ends.get(&edge_start)) {
                        num_sides += 1;
                    }
                    vert_edge_starts.insert(edge_start);
                    vert_edge_ends.insert(edge_end);
                },
                _ => (),
            }
        }
    }

    // Horiz edges
    for col in min_col..=max_col {
        for row in min_row - 1..=max_row {
            match (plots.get(&(row, col)), plots.get(&(row + 1, col))) {
                (None, Some(_)) => {
                    let (edge_start, edge_end) = ((row + 1, col, EdgeType::Entry), (row + 1, col + 1, EdgeType::Entry));
                    if let (None, None) = (horiz_edge_starts.get(&edge_end), horiz_edge_ends.get(&edge_start)) {
                        num_sides += 1;
                    }
                    horiz_edge_starts.insert(edge_start);
                    horiz_edge_ends.insert(edge_end);
                },
                (Some(_), None) => {
                    let (edge_start, edge_end) = ((row + 1, col, EdgeType::Exit), (row + 1, col + 1, EdgeType::Exit));
                    if let (None, None) = (horiz_edge_starts.get(&edge_end), horiz_edge_ends.get(&edge_start)) {
                        num_sides += 1;
                    }
                    horiz_edge_starts.insert(edge_start);
                    horiz_edge_ends.insert(edge_end);
                },
                _ => (),
            }
        }
    }

    num_sides * region.len() as u64
}

fn solve_part_two(data: &str) -> u64 {
    let mut regions = parse_grid(data);
    regions
        .values_mut()
        .map(|region| calc_discount_price(region))
        .sum()
}

fn main() {
    let data = std::fs::read_to_string("day12/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "AAAA\n\
                         BBCD\n\
                         BBCC\n\
                         EEEC";

    #[test]
    fn day12_part_one() {
        assert_eq!(solve_part_one(INPUT), 140);
    }

    #[test]
    fn day12_part_two() {
        assert_eq!(solve_part_two(INPUT), 80);
    }

    const INPUT_2: &str = "OOOOO\n\
                           OXOXO\n\
                           OOOOO\n\
                           OXOXO\n\
                           OOOOO";

    #[test]
    fn day12_part_one_input_2() {
        assert_eq!(solve_part_one(INPUT_2), 772);
    }

    #[test]
    fn day12_part_two_input_2() {
        assert_eq!(solve_part_two(INPUT_2), 436);
    }

    const INPUT_3: &str = "RRRRIICCFF\n\
                           RRRRIICCCF\n\
                           VVRRRCCFFF\n\
                           VVRCCCJFFF\n\
                           VVVVCJJCFE\n\
                           VVIVCCJJEE\n\
                           VVIIICJJEE\n\
                           MIIIIIJJEE\n\
                           MIIISIJEEE\n\
                           MMMISSJEEE";

    #[test]
    fn day12_part_one_input_3() {
        assert_eq!(solve_part_one(INPUT_3), 1930);
    }

    #[test]
    fn day12_part_two_input_3() {
        assert_eq!(solve_part_two(INPUT_3), 1206);
    }

    const INPUT_4: &str = "EEEEE\n\
                           EXXXX\n\
                           EEEEE\n\
                           EXXXX\n\
                           EEEEE";
    #[test]
    fn day12_part_two_input_4() {
        assert_eq!(solve_part_two(INPUT_4), 236);
    }

    const INPUT_5: &str = "AAAAAA\n\
                           AAABBA\n\
                           AAABBA\n\
                           ABBAAA\n\
                           ABBAAA\n\
                           AAAAAA";
    #[test]
    fn day12_part_two_input_5() {
        assert_eq!(solve_part_two(INPUT_5), 368);
    }

}
