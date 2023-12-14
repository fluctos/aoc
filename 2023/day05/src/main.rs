use std::collections::HashMap;

type Range = (usize, usize);
type Mapper = HashMap<Range, usize>;

fn map_range(range: &Range, mapper: &Mapper) -> Vec<Range> {
    let mut ranges_out = Vec::<Range>::new();
    let (mut range_min, range_max) = *range;
    let mut relevant: Vec<&Range> = mapper.keys().filter(|r| r.0 < range_max && r.1 > range_min).collect();
    relevant.sort_by(|&(a_min, _), &(b_min, _)| a_min.cmp(b_min));

    for &map_range in relevant {
        let (map_range_min, map_range_max) = map_range;
        let dst_range_min = *mapper.get(&map_range).unwrap();
        let discriminator = (usize::min(range_min, map_range_min) == range_min,
                             usize::min(range_max, map_range_max) == range_max);
        match discriminator {
            (true,  true ) => {
                // range overlaping map_range from below
                if range_min != map_range_min {
                    ranges_out.push((range_min, map_range_min));
                }
                ranges_out.push((dst_range_min, dst_range_min + range_max - map_range_min));
                range_min = range_max;
                break;
            }

            (false, false) => {
                // range overlaping map_range from above
                ranges_out.push((dst_range_min + range_min - map_range_min, dst_range_min + map_range_max - map_range_min));
                range_min = map_range_max;
            }

            (false, true)  => {
                // range fully contained in map_range
                ranges_out.push((dst_range_min + range_min - map_range_min, dst_range_min + range_max - map_range_min));
                range_min = range_max;
                break;
            }

            (true,  false) => {
                // map_range fully contained in range
                ranges_out.push((range_min, map_range_min));
                ranges_out.push((dst_range_min, dst_range_min + map_range_max - map_range_min));
                range_min = map_range_max;
            }
        }
    }

    if range_min != range_max {
        ranges_out.push((range_min, range_max));
    }

    ranges_out
}

fn find_min_mapping(ranges: &Vec<Range>, mappers: &Vec<Mapper>) -> usize {
    let mut ranges_in = ranges.clone();
    let mut ranges_out = Vec::<Range>::new();

    for mapper in mappers {
        for range in ranges_in {
            ranges_out.append(&mut map_range(&range, mapper));
        }
        ranges_in = ranges_out;
        ranges_out = Vec::new();
    }

    ranges_in.iter().map(|rng| rng.0).min().unwrap()
}

fn solution(input_file_path: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut sections = data.split("\n\n");

    let raw_seeds: Vec<usize> = sections
        .next().unwrap()
        .split_once(':').unwrap().1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mappers: Vec<Mapper> = sections.map(|s| {
        s.lines().skip(1).map(|l| {
            let range_def: Vec<usize> = l.split_whitespace().map(|nstr| nstr.parse::<usize>().unwrap()).collect();
            let (dst, src, nelems) = (range_def[0], range_def[1], range_def[2]);
            ((src, src + nelems), dst)
        }).collect::<Mapper>()
    }).collect();


    let point_seeds: Vec<_> = raw_seeds.iter().map(|&s| (s, s + 1)).collect();
    let range_seeds: Vec<_> = raw_seeds.chunks(2).map(|c| (c[0], c[0] + c[1])).collect();

    (find_min_mapping(&point_seeds, &mappers), find_min_mapping(&range_seeds, &mappers))
}

fn main() {
    println!("test:  {:?}", solution("day05/input/test.txt"));
    println!("input: {:?}", solution("day05/input/input.txt"));
}

