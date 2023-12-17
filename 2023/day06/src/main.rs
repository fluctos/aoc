use std::iter::zip;

fn solve(time: usize, distance: usize) -> usize {
    /* s - distance
     * t - time
     * d - delay
     *
     * s(t) = max(0, dt - d^2)
     *
     * To 'beat' the record for given s0, t0
     *
     * s(t0) > s0
     *
     * As a function of d:
     *
     * -d^2 + t0 * d - s0 > 0
     *
     * To solve, find all integer solutions of this quadratic inequality.
     */

    let mut num_strategies = 0usize;

    let delta = time.pow(2) as f64 - 4.0 * distance as f64;

    if delta > 0.0 {
        let d0 = (-1.0f64 * time as f64 - delta.sqrt()) / -2.0;
        let d1 = (-1.0f64 * time as f64 + delta.sqrt()) / -2.0;

        let mut beg = f64::min(d0, d1);
        let mut end = f64::max(d0, d1);

        if beg.fract() == 0.0f64 {
            beg += 1.0f64;
        }

        if end.fract() == 0.0f64 {
            end -= 1.0f64;
        }

        num_strategies = f64::floor(end) as usize - f64::ceil(beg) as usize + 1;
    }

    num_strategies
}

fn solution(input_file_path: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(input_file_path).unwrap();

    let values: Vec<Vec<usize>> = data.lines().map(|l| {
        l.split_once(":").unwrap().1.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect()
    }).collect();

    let mut product = 1usize;
    for (&time, &distance) in zip(&values[0], &values[1]){
       product *= solve(time, distance);
    }

    let total_time     = values[0].iter().fold(0usize, |acc, elem| acc * 10usize.pow(elem.ilog10() + 1) + elem);
    let total_distance = values[1].iter().fold(0usize, |acc, elem| acc * 10usize.pow(elem.ilog10() + 1) + elem);

    (product, solve(total_time, total_distance))
}

fn main() {
    println!("test:  {:?}", solution("day06/input/test.txt"));
    println!("input: {:?}", solution("day06/input/input.txt"));
}

