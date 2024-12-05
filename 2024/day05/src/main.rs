use std::collections::{HashMap, HashSet};

fn preprocess(data: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let (rule_list, update_list) = data.split_once("\n\n").unwrap();

    let rules: HashMap<u64, HashSet<u64>> = rule_list
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .fold(HashMap::new(), |mut map, (a, b)| {
            map.entry(a)
               .or_insert(HashSet::new())
               .insert(b);

            map
        });

    let updates: Vec<Vec<u64>> = update_list
        .lines()
        .map(|line| {
            line.split(",")
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    (rules, updates)
}

fn is_update_correct(update: &Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> Result<(), (usize, usize)> {
    let mut numbers_before = HashSet::<u64>::new();
    let mut indices_before = HashMap::<u64, usize>::new();

    for (idx, number) in update.iter().enumerate() {

        if let Some(numbers_after) = rules.get(number) {
            if !numbers_before.is_disjoint(numbers_after) {
                let idx_current = idx;
                let idx_before = numbers_before
                    .intersection(&numbers_after)
                    .filter_map(|n| indices_before.get(n))
                    .min()
                    .unwrap();

                return Err((idx_current, *idx_before));
            }
        }

        numbers_before.insert(*number);
        indices_before.insert(*number, idx);
    }

    Ok(())
}

fn solve_part_one(data: &str) -> u64 {
    let (rules, updates) = preprocess(data);

    let mut result = 0u64;
    for update in updates.iter() {
        if is_update_correct(&update, &rules).is_ok() {
            result += update.get(update.len() / 2).unwrap();
        }
    }

    result
}

fn solve_part_two(data: &str) -> u64 {
    let (rules, mut updates) = preprocess(data);

    let mut result = 0u64;
    for update in updates.iter_mut() {
        let mut modified = false;
        while let Err((idx_a, idx_b)) = is_update_correct(&update, &rules) {
            update.swap(idx_a, idx_b);
            modified = true;
        }
        if modified {
            result += update.get(update.len() / 2).unwrap();
        }
    }

    result
}

fn main() {
    let data = std::fs::read_to_string("day05/input.txt").unwrap();

    println!("Part one: {}", solve_part_one(data.as_str()));
    println!("Part two: {}", solve_part_two(data.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "47|53\n\
                         97|13\n\
                         97|61\n\
                         97|47\n\
                         75|29\n\
                         61|13\n\
                         75|53\n\
                         29|13\n\
                         97|29\n\
                         53|29\n\
                         61|53\n\
                         97|53\n\
                         61|29\n\
                         47|13\n\
                         75|47\n\
                         97|75\n\
                         47|61\n\
                         75|61\n\
                         47|29\n\
                         75|13\n\
                         53|13\n\
                         \n\
                         75,47,61,53,29\n\
                         97,61,53,29,13\n\
                         75,29,13\n\
                         75,97,47,61,53\n\
                         61,13,29\n\
                         97,13,75,29,47";
    #[test]
    fn day05_part_one() {
        assert_eq!(solve_part_one(INPUT), 143);
    }

    #[test]
    fn day05_part_two() {
        assert_eq!(solve_part_two(INPUT), 123);
    }
}
