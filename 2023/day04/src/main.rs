use std::collections::HashSet;

fn part_a(stats: &Vec<u32>) -> u32 {
    stats.iter().map(|s| {
        match s {
            0 => 0u32,
            n => 2u32.pow(n - 1),
        }
    }).sum()
}

fn traverse(stats: &Vec<u32>, index: usize, cards: &mut u32) {
    let num_wins = stats[index];
    *cards += 1;
    for new_idx in index + 1 .. index + 1 + num_wins as usize {
        traverse(stats, new_idx, cards);
    }
}

fn part_b(stats: &Vec<u32>) -> u32 {
    let mut cards = 0u32;
    for idx in 0usize..stats.len() {
        traverse(&stats, idx, &mut cards);
    }
    cards
}

fn solution(input_file_path: &str) -> (u32, u32) {
    let data = std::fs::read_to_string(input_file_path).unwrap();

    let win_stats: Vec<u32> = data.lines().map(|l| {
        let numbers = l.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: HashSet<u32> = numbers.0.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        let scratched: HashSet<u32> = numbers.1.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        let matched: Vec<&u32> = winning.intersection(&scratched).collect();

        matched.len() as u32
    })
    .collect();

    (part_a(&win_stats), part_b(&win_stats))
}

fn main() {
    println!("test:  {:?}", solution("day04/input/test.txt"));
    println!("input: {:?}", solution("day04/input/input.txt"));
}

