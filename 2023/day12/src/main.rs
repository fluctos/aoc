use std::iter::once;
use std::collections::HashMap;

fn get_arrangements<'a, 'b>(template: &'a [char], groups: &'a [usize], cache: &'b mut HashMap<(&'a [char], &'a [usize]), usize>) -> usize {

    if !groups.is_empty() && template.len() < groups.iter().sum::<usize>() + groups.len() - 1 {
        return 0;
    }

    if groups.is_empty() && template.contains(&'#') {
        return 0;
    }

    if groups.is_empty() && !template.contains(&'#') {
        return 1;
    }

    if cache.contains_key(&(template, groups)) {
        return cache.get(&(template, groups)).unwrap().clone();
    }

    let mut num_arrangements = 0usize;

    if ['.', '?'].contains(&template[0]) {
        num_arrangements += get_arrangements(&template[1..], groups, cache);
    }

    if ['#', '?'].contains(&template[0]) {
        let dotpos = template.iter().position(|&c| c == '.');
        if dotpos.is_none() || dotpos.unwrap() >= groups[0] {
            if template.len() > groups[0] {
                if ['.', '?'].contains(&template[groups[0]]) {
                    num_arrangements += get_arrangements(&template[groups[0] + 1..], &groups[1..], cache);
                }
            } else {
                num_arrangements += get_arrangements(&template[groups[0]..], &groups[1..], cache);
            }
        }
    }

    cache.insert((template, groups), num_arrangements);

    num_arrangements
}

fn solution(input_file_path: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut part_a = 0usize;
    let mut part_b = 0usize;

    for line in data.lines() {
        let tokens = line.split_once(' ').unwrap();
        let template: Vec<char> = tokens.0.chars().collect();
        let groups: Vec<usize> = tokens.1.split(',').map(|d| d.parse::<usize>().unwrap()).collect();

        let mut cache: HashMap<(&[char], &[usize]), usize> = HashMap::new();

        part_a += get_arrangements(&template, &groups, &mut cache);

        let expanded_template: Vec<char> = template.iter().chain(once(&'?')).cycle().take(template.len() * 5 + 4).cloned().collect();
        let expanded_groups: Vec<usize> = groups.iter().cycle().take(groups.len() * 5).cloned().collect();

        part_b += get_arrangements(&expanded_template, &expanded_groups, &mut cache);
    }

    (part_a, part_b)
}

fn main() {
    println!("{:?}", solution("day12/input/test.txt"));
    println!("{:?}", solution("day12/input/input.txt"));
}

