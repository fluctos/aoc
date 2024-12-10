
fn solve_part_one(data: &str) -> u64 {
    let num_blocks: usize = data
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();

    let iter = data
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .enumerate()
        .flat_map(|(idx, num)| {
            match idx % 2 {
                0 => std::iter::repeat(Some(idx / 2)).take(num as usize),
                1 => std::iter::repeat(None).take(num as usize),
                _ => unreachable!(),
            }
        })
        .enumerate();

    let mut rev_iter = data
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .rev()
        .enumerate()
        .flat_map(|(idx, num)| {
            let rev_idx = data.len() - 1 - idx;
            match rev_idx % 2 {
                0 => std::iter::repeat(Some(rev_idx / 2)).take(num as usize),
                1 => std::iter::repeat(None).take(num as usize),
                _ => unreachable!(),
            }
        })
        .enumerate()
        .map(|(idx, opt)| (num_blocks - 1 - idx, opt))
        .filter(|(_, opt)| opt.is_some());

    let mut non_empty_blocks = rev_iter.clone().count();
    let mut result = 0u64;

    for item in iter {
        match item {
            (idx, Some(block_id)) => {
                result += idx as u64 * block_id as u64;
                non_empty_blocks -= 1;
            },
            (idx, None) => {
                loop {
                    match rev_iter.next() {
                        Some((_, Some(rev_block_id))) => {
                            result += idx as u64 * rev_block_id as u64;
                            non_empty_blocks -= 1;
                            break;
                        },
                        None => break,
                        _ => unreachable!(),
                    }
                }
            },
        }

        if non_empty_blocks == 0 {
            break;
        }
    }

    result
}

fn solve_part_two(data: &str) -> u64 {
    0
}

fn main() {
    let data = std::fs::read_to_string("day09/input.txt").unwrap();
    let data = data.trim();

    println!("Part one: {}", solve_part_one(data));
    println!("Part two: {}", solve_part_two(data));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn day09_part_one() {
        assert_eq!(solve_part_one(INPUT), 1928);
    }

    #[test]
    fn day09_part_two() {
        assert_eq!(solve_part_two(INPUT), 0);
    }
}
