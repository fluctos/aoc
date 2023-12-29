fn interpolate(history: &Vec<i32>) -> (i32, i32) {
    if history.iter().all(|&v| v == 0) {
        return (0, 0);
    }

    assert!(history.len() > 1);

    let diffs: Vec<i32> = history.windows(2).map(|w| w[1] - w[0]).collect();
    let interp = interpolate(&diffs);

    (history.first().unwrap() - interp.0, history.last().unwrap() + interp.1)
}

fn solution(input_file_path: &str) -> (i32, i32) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut log: Vec<Vec<i32>> = Vec::new();
    for line in data.lines() {
        log.push(
            line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()
        );
    }

    for value_history in &mut log {
        let interp = interpolate(&value_history);
        value_history.insert(0, interp.0);
        value_history.push(interp.1);
    }

    (log.iter().map(|h| h.first().unwrap()).sum(), log.iter().map(|h| h.last().unwrap()).sum())
}

fn main() {
    println!("{:?}", solution("day09/input/test.txt"));
    println!("{:?}", solution("day09/input/input.txt"));
}

