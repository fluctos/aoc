use std::str::FromStr;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Rule {
    field: char,
    op: char,
    value: u64,
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Rule, Self::Err> {
        let mut chars = s.chars();
        Ok(Rule{
            field: chars.next().unwrap(),
            op:    chars.next().unwrap(),
            value: chars.as_str().parse().unwrap(),
        })
    }
}

type Output<'a> = (Option<Rule>, &'a str);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Sieve {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl Sieve {
    fn new() -> Self {
        Self{
            x: (0, 4001),
            m: (0, 4001),
            a: (0, 4001),
            s: (0, 4001),
        }
    }

    fn from(x: (u64, u64), m: (u64, u64), a: (u64, u64), s:(u64, u64)) -> Self {
        Self{x, m, a, s}
    }

    fn restrict(&self, rule: &Rule) -> Self {
        let op = match rule.op {
            '<' => |(min, max), val| (min, u64::min(max, val)),
            '>' => |(min, max), val| (u64::max(min, val), max),
            _ => unreachable!(),
        };
        match rule.field {
            'x' => Self::from(op(self.x, rule.value), self.m, self.a, self.s),
            'm' => Self::from(self.x, op(self.m, rule.value), self.a, self.s),
            'a' => Self::from(self.x, self.m, op(self.a, rule.value), self.s),
            's' => Self::from(self.x, self.m, self.a, op(self.s, rule.value)),
            _ => unreachable!(),
        }
    }

    fn exclude(&self, rule: &Rule) -> Self {
        let op = match rule.op {
            '<' => |(min, max), val| (u64::max(min, val - 1), max),
            '>' => |(min, max), val| (min, u64::min(max, val + 1)),
            _ => unreachable!(),
        };
        match rule.field {
            'x' => Self::from(op(self.x, rule.value), self.m, self.a, self.s),
            'm' => Self::from(self.x, op(self.m, rule.value), self.a, self.s),
            'a' => Self::from(self.x, self.m, op(self.a, rule.value), self.s),
            's' => Self::from(self.x, self.m, self.a, op(self.s, rule.value)),
            _ => unreachable!(),
        }
    }
}


#[derive(Debug)]
struct Node<'a> {
    inputs: HashSet<Sieve>,
    outputs: Vec<Output<'a>>
}

impl Node<'_> {
    fn new() -> Self {
        Self{inputs: HashSet::new(), outputs: Vec::new()}
    }
}

type Graph<'a> = HashMap<&'a str, Node<'a>>;

fn parse_rule(rule: &str) -> Output {
    if let Some((condition, name)) = rule.split_once(':') {
        (Some(Rule::from_str(condition).unwrap()), name)
    } else {
        (None, rule)
    }
}

fn build_graph(data: &str) -> Graph {
    let mut graph = Graph::new();
    for line in data.lines() {
        let (node_name, rules) = line[0..line.len()-1].split_once('{').unwrap();
        let node = graph.entry(node_name).or_insert(Node::new());
        for rule in rules.split(',') {
            node.outputs.push(parse_rule(rule));
        }
    }

    graph.insert("A", Node::new());
    graph.insert("R", Node::new());

    graph
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

impl Part {
    fn new(x:u64, m:u64, a:u64, s:u64) -> Self {
        Self {x, m, a, s}
    }
}

impl FromStr for Part {
    type Err = ();
    fn from_str(s: &str) -> Result<Part, Self::Err> {
        let values: Vec<u64> = s[1..s.len()-1]
            .split(',')
            .map(|eq| eq.split_once("=").unwrap().1.parse().unwrap())
            .collect();
        Ok(Part::new(values[0], values[1], values[2], values[3]))
    }
}

fn get_parts(data: &str) -> Vec<Part> {
    data.lines().map(|line| Part::from_str(line).unwrap()).collect()
}

fn calc_sieves(graph: &mut Graph) {
    let mut candidates: VecDeque<(Sieve, &str)> = VecDeque::new();
    candidates.push_back((Sieve::new(), "in"));
    while let Some((mut sieve, name)) = candidates.pop_front() {
        let node = graph.get_mut(name).unwrap();
        node.inputs.insert(sieve.clone());
        for output in node.outputs.iter() {
            match output {
                (Some(rule), name) => {
                    candidates.push_back((sieve.restrict(rule), name));
                    sieve = sieve.exclude(rule);
                },
                (None, name) => {
                    candidates.push_back((sieve, name));
                },
            }
        }
    }
}

fn calc_part_1(graph: &Graph, parts: &Vec<Part>) -> u64 {
    let a_node = graph.get("A").unwrap();
    let mut count = 0u64;

    for sieve in a_node.inputs.iter() {
        for part in parts {
            if part.x > sieve.x.0 && part.x < sieve.x.1 &&
               part.m > sieve.m.0 && part.m < sieve.m.1 &&
               part.a > sieve.a.0 && part.a < sieve.a.1 &&
               part.s > sieve.s.0 && part.s < sieve.s.1 {
                count += part.x + part.m + part.a + part.s;
            }
        }
    }

    count
}

fn calc_part_2(graph: &Graph) -> u64 {
    let a_node = graph.get("A").unwrap();
    let mut combinations = 0u64;
    for input in a_node.inputs.iter() {
        let mut current = 0u64;
        if input.x.0 < input.x.1 { current  = input.x.1 - input.x.0 - 1; }
        if input.m.0 < input.m.1 { current *= input.m.1 - input.m.0 - 1; }
        if input.a.0 < input.a.1 { current *= input.a.1 - input.a.0 - 1; }
        if input.s.0 < input.s.1 { current *= input.s.1 - input.s.0 - 1; }
        combinations += current;
    }

    combinations
}

fn process_input(input_file_path: &str) -> (u64, u64) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut sections = data.split("\n\n");

    let mut graph = build_graph(sections.next().unwrap());
    let parts = get_parts(sections.next().unwrap());

    calc_sieves(&mut graph);

    (calc_part_1(&graph, &parts), calc_part_2(&graph))
}

fn main() {
    println!("{:?}", process_input("day19/input/test.txt"));
    println!("{:?}", process_input("day19/input/input.txt"));
}

