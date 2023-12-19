use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

fn get_hand_type(hand: &([char; 5], u32), use_jokers: bool) -> HandType {
    let mut counter: HashMap<char, u32> = HashMap::new();

    for chr in hand.0 {
        counter.entry(chr).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut num_jokers = 0u32;

    if use_jokers {
        match counter.remove(&'J') {
            Some(5) => return HandType::FiveOfAKind,
            Some(n) => num_jokers = n,
            None => (),
        }
    }

    let mut counts:Vec<(char, u32)> = counter.drain().collect();
    counts.sort_by(|(_, a_val), (_, b_val)| b_val.cmp(a_val));

    if use_jokers {
        counts[0].1 += num_jokers;
    }

    match counts[0].1 {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => match counts[1].1 {
            2 => HandType::FullHouse,
            _ => HandType::ThreeOfAKind,
        },
        2 => match counts[1].1 {
            2 => HandType::TwoPair,
            _ => HandType::OnePair,
        },
        _ => HandType::HighCard,
    }
}

fn get_label_value(chr: &char, use_jokers: bool) -> u32 {
    match chr {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => if use_jokers { 1 } else { 11 },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!(),
    }
}

fn compare(a: &[char; 5], b: &[char;5], use_jokers: bool) -> Ordering {
    let a_values: Vec<u32> = a.iter().map(|chr| get_label_value(chr, use_jokers)).collect();
    let b_values: Vec<u32> = b.iter().map(|chr| get_label_value(chr, use_jokers)).collect();

    a_values.cmp(&b_values)
}

fn solution(input_file_path: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let hands: Vec<([char;5], u32)> = data.lines().map(|line| {
        let tokens = line.split_once(' ').unwrap();
        (tokens.0.chars().take(5).collect::<Vec<char>>().try_into().unwrap(), tokens.1.parse::<u32>().unwrap())
    }).collect();

    let mut hands_no_jokers: Vec<([char;5], u32, HandType)> = hands.iter().map(|hand| (hand.0, hand.1, get_hand_type(hand, false))).collect();
    let mut hands_with_jokers: Vec<([char;5], u32, HandType)> = hands.iter().map(|hand| (hand.0, hand.1, get_hand_type(hand, true))).collect();

    hands_no_jokers.sort_by(|a, b| {
        match a.2.cmp(&b.2) {
            Ordering::Equal => compare(&a.0, &b.0, false),
            ord => ord,
        }
    });

    hands_with_jokers.sort_by(|a, b| {
        match a.2.cmp(&b.2) {
            Ordering::Equal => compare(&a.0, &b.0, true),
            ord => ord,
        }
    });

    let part_a = hands_no_jokers.iter().enumerate().fold(0usize, |acc, elem| acc + (elem.0 + 1) * elem.1.1 as usize);
    let part_b = hands_with_jokers.iter().enumerate().fold(0usize, |acc, elem| acc + (elem.0 + 1) * elem.1.1 as usize);

    (part_a, part_b)
}

fn main() {
    println!("test:  {:?}", solution("day07/input/test.txt"));
    println!("input: {:?}", solution("day07/input/input.txt"));
}

