fn hash(s: &str) -> u8 {
    let mut hash = 0u64;
    for chr in s.chars() {
        hash += chr as u64;
        hash *= 17;
        hash %= 256;
    }
    hash as u8
}

fn insert_or_update<'a, 'b>(buckets: &'a mut Vec<Vec<(&'b str, u64)>>, label: &'b str, length: u64) {
    let index = hash(label) as usize;
    let bucket = buckets.get_mut(index).unwrap();
    match bucket.iter().position(|(l, _)| l == &label) {
        None => bucket.push((label, length)),
        Some(i) => bucket[i] = (label, length),
    }
}

fn remove(buckets: &mut Vec<Vec<(&str, u64)>>, label: &str) {
    let index = hash(label) as usize;
    let bucket = buckets.get_mut(index).unwrap();
    if let Some(i) = bucket.iter().position(|(l, _)| l == &label) {
        bucket.remove(i);
    }
}

fn eval(buckets: &Vec<Vec<(&str, u64)>>) -> u64 {
    let mut score = 0u64;
    for (bucket_index, bucket) in buckets.iter().enumerate() {
        for (slot_index, (_, length)) in bucket.iter().enumerate() {
            score += (bucket_index as u64 + 1) * (slot_index as u64 + 1) * length;
        }
    }

    score
}

fn solution(input_file_path: &str) -> (u64, u64) {
    let data = std::fs::read_to_string(input_file_path).unwrap().replace('\n', "").replace('\r', "");

    let part_a = data.split(',').map(|token| hash(token) as u64).sum();

    let mut buckets: Vec<Vec<(&str, u64)>> = (0..256).map(|_| Vec::new()).collect();
    for init_step in data.split(',') {
        if let Some(tokens) = init_step.split_once('=') {
            let label = tokens.0;
            let length = tokens.1.parse::<u64>().unwrap();
            insert_or_update(&mut buckets, label, length);
        }
        if let Some((label, _)) = init_step.split_once('-') {
            remove(&mut buckets, label);
        }
    }

    let part_b = eval(&buckets);

    (part_a, part_b)
}

fn main() {
    println!("{:?}", solution("day15/input/test.txt"));
    println!("{:?}", solution("day15/input/input.txt"));
}
