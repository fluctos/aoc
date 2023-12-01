fn valueof(s: &str) -> u32 {
    match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0
    }
}

fn find_first_value(line: &str, patterns: &[&str]) -> u32 {
    let mut matches: Vec<(usize, &str)> = patterns
        .iter()
        .map(|p| (line.find(*p), *p))
        .filter(|t| t.0.is_some())
        .map(|t| (t.0.unwrap(), t.1))
        .collect();

    matches.sort();

    match matches.len() {
        0 => 0,
        _ => valueof(matches[0].1)
    }
}

fn find_last_value(line: &str, patterns: &[&str]) -> u32 {
    let mut matches: Vec<(usize, &str)> = patterns
        .iter()
        .map(|p| (line.rfind(*p), *p))
        .filter(|t| t.0.is_some())
        .map(|t| (t.0.unwrap(), t.1))
        .collect();

    matches.sort();

    match matches.len() {
        0 => 0,
        n => valueof(matches[n-1].1)
    }
}

fn solution(path: &str, patterns: &[&str]) -> u32 {
    let data = std::fs::read_to_string(path).unwrap();
    let lines = data.lines();
    let mut sum = 0u32;
    for line in lines {
        let first = find_first_value(&line, &patterns);
        let last = find_last_value(&line, &patterns);
        sum += first * 10 + last;
    }
    sum
}

fn main() {

    let digits: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let words: &[&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let patterns_a = &digits;
    let patterns_b = &[digits, words].concat();

    println!("[A] test  {}", solution("day01/input/test_a.txt", &patterns_a));
    println!("[A] input {}", solution("day01/input/input.txt",  &patterns_a));
    println!("[B] test  {}", solution("day01/input/test_b.txt", &patterns_b));
    println!("[B] input {}", solution("day01/input/input.txt",  &patterns_b));
}
