use std::collections::HashMap;
use std::iter::zip;

fn traverse(directions: &Vec<char>, lr_map: &HashMap<&str, (&str, &str)>, from_node: &str) -> usize {
    let mut node = from_node;
    for (index, turn) in directions.iter().cycle().enumerate() {
        match turn {
            'L' => node = lr_map[node].0,
             _  => node = lr_map[node].1,
        }
        if node.ends_with('Z') {
            return index + 1;
        }
    }

    0
}

fn gcd(a: &usize, b: &usize) -> usize {
    assert!(*a != 0 && *b != 0);
    let mut m = a.clone();
    let mut n = b.clone();
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }

    n
}

fn lcm(a: &usize, b: &usize) -> usize {
    a * b / gcd(a, b)
}

fn solution(input_file_path: &str) -> usize {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut lines = data.lines();

    let directions: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next().unwrap();

    let mut lr_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let (from, to) = line.split_once(" = ").unwrap();
        let (to_left, to_right) = to[1 .. to.len()-1].split_once(", ").unwrap();
        lr_map.insert(from, (to_left, to_right));
    }

    let ghosts: Vec<&str> = lr_map.keys().filter(|k| k.ends_with('A')).map(|&k| k).collect();
    let cycles: Vec<usize> = ghosts.iter().map(|&g| traverse(&directions, &lr_map, g)).collect();

    println!("{:?}", zip(&ghosts, &cycles).collect::<Vec<_>>());

    match ghosts.len() {
        0 => 0,
        1 => cycles[0],
        _ => cycles.iter().fold(cycles[0], |acc, elem| lcm(&acc, &elem))
    }
}

fn main() {
    println!("test_a: {:?}", solution("day08/input/test_a.txt"));
    println!("test_b: {:?}", solution("day08/input/test_b.txt"));
    println!("test_c: {:?}", solution("day08/input/test_c.txt"));
    println!("input:  {:?}", solution("day08/input/input.txt"));
}

