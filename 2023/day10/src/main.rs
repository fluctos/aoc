use std::collections::HashSet;

fn get_neighbors(sketch: &Vec<Vec<char>>, row: usize, col: usize) -> (char, char, char, char) {
    let num_rows = sketch.len();
    let num_cols = sketch[0].len();

    let n_char = match row {
        0 => '.',
        _ => sketch[row - 1][col],
    };

    let e_char = match col {
        n if n >= num_cols - 1 => '.',
        _ => sketch[row][col + 1],
    };

    let s_char = match row {
        n if n >= num_rows - 1 => '.',
        _ => sketch[row + 1][col],
    };

    let w_char = match col {
        0 => '.',
        _ => sketch[row][col - 1],
    };

    (n_char, e_char, s_char, w_char)
}

fn traverse(sketch: &Vec<Vec<char>>, s_row: usize, s_col: usize) -> Vec<(usize, usize)> {
    let mut steps: Vec<(usize, usize)> = Vec::new();
    let mut pos = (s_row, s_col);
    let mut direction = match sketch[pos.0][pos.1] {
        '|' => 'N',
        '-' => 'W',
        'L' => 'E',
        'J' => 'W',
        '7' => 'E',
        'F' => 'N',
        _ => panic!(),
    };

    loop {
        steps.push(pos);
        direction = match (sketch[pos.0][pos.1], direction) {
            ('|', 'N') => 'N',
            ('|', 'S') => 'S',
            ('-', 'E') => 'E',
            ('-', 'W') => 'W',
            ('L', 'S') => 'E',
            ('L', 'W') => 'N',
            ('J', 'S') => 'W',
            ('J', 'E') => 'N',
            ('7', 'E') => 'S',
            ('7', 'N') => 'W',
            ('F', 'N') => 'E',
            ('F', 'W') => 'S',
            _ => panic!(),
        };
        pos = match direction {
            'N' => (pos.0 - 1, pos.1),
            'E' => (pos.0,     pos.1 + 1),
            'S' => (pos.0 + 1, pos.1),
            'W' => (pos.0,     pos.1 - 1),
            _ => panic!(),
        };
        if pos == (s_row, s_col) {
            break;
        }
    }

    steps
}

enum ScanState {
    Normal,
    EdgeFromAbove,
    EdgeFromBelow,
}

fn scan(sketch: &Vec<Vec<char>>, pipeline: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut enclosed: Vec<(usize, usize)> = Vec::new();
    let pipes: HashSet<(usize, usize)> = pipeline.iter().cloned().collect();
    for (row, line) in sketch.iter().enumerate() {
        let mut inside = false;
        let mut state: ScanState = ScanState::Normal;
        for (col, chr) in line.iter().enumerate() {
            if pipes.contains(&(row, col)) {
                (inside, state) = match (chr, &inside, &state) {
                    ('|', _, _)                        => (!inside, ScanState::Normal),
                    ('F', _, _)                        => ( inside, ScanState::EdgeFromBelow),
                    ('L', _, _)                        => ( inside, ScanState::EdgeFromAbove),
                    ('-', _, _)                        => ( inside, state),
                    ('7', _, ScanState::EdgeFromAbove) => (!inside, ScanState::Normal),
                    ('7', _, ScanState::EdgeFromBelow) => ( inside, ScanState::Normal),
                    ('J', _, ScanState::EdgeFromAbove) => ( inside, ScanState::Normal),
                    ('J', _, ScanState::EdgeFromBelow) => (!inside, ScanState::Normal),
                    _ => panic!(),
                };
            } else {
                if inside {
                    enclosed.push((row, col));
                }
            }
        }
    }

    enclosed
}

fn solution(input_file_path: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut sketch: Vec<Vec<char>> = Vec::new();
    let mut s_row = 0usize;
    let mut s_col = 0usize;
    for (row, line) in data.lines().enumerate() {
        sketch.push(line.chars().collect());
        if let Some(col) = line.find('S') {
            s_row = row;
            s_col = col;
        }
    }

    // N -> E -> S -> W
    // | - L J 7 F

    let s_char = match get_neighbors(&sketch, s_row, s_col) { // (sketch[s_row - 1][s_col], sketch[s_row][s_col + 1], sketch[s_row + 1][s_col], sketch[s_row][s_col -1]) {
        ('F'|'|'|'7', _, 'J'|'|'|'L', _) => '|',
        (_, 'J'|'-'|'7', _, 'L'|'-'|'F') => '-',
        ('F'|'|'|'7', 'J'|'-'|'7', _, _) => 'L',
        ('F'|'|'|'7', _, _, 'L'|'-'|'F') => 'J',
        (_, _, 'J'|'|'|'L', 'L'|'-'|'F') => '7',
        (_, 'J'|'-'|'7', 'J'|'|'|'L', _) => 'F',
        _ => panic!(),
    };

    sketch[s_row][s_col] = s_char;

    let pipeline = traverse(&sketch, s_row, s_col);
    let enclosed = scan(&sketch, &pipeline);

    (pipeline.len() / 2, enclosed.len())
}

fn main() {
    println!("{:?}", solution("day10/input/test_a.txt"));
    println!("{:?}", solution("day10/input/test_b.txt"));
    println!("{:?}", solution("day10/input/test_c.txt"));
    println!("{:?}", solution("day10/input/test_d.txt"));
    println!("{:?}", solution("day10/input/test_e.txt"));
    println!("{:?}", solution("day10/input/input.txt"));
}

