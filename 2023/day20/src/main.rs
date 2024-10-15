use std::collections::{HashMap, VecDeque};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
enum Level {
    Low,
    High,
}

#[derive(Debug)]
struct Signal {
    from: String,
    to: String,
    level: Level,
}

impl Signal {
    fn new(from: &String, to: &String, level: Level) -> Self {
        Self{from: from.clone(), to: to.clone(), level}
    }
}

trait Module: Any {
    fn activate(&mut self, signal: Signal) -> Vec<Signal>;
    fn register_input(&mut self, name: &String);
    fn register_output(&mut self, name: &String);
    fn reset(&mut self);
    fn as_any(&mut self) -> &mut dyn Any;
}

struct FlipFlop {
    name: String,
    level: Level,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl FlipFlop {
    fn new(name: &String) -> Self {
        Self{name: name.clone(), level: Level::Low, inputs: Vec::new(), outputs: Vec::new()}
    }

    fn flip(&mut self) {
        self.level = match self.level {
            Level::Low => Level::High,
            Level::High => Level::Low,
        }
    }
}

impl Module for FlipFlop {
    fn activate(&mut self, signal: Signal) -> Vec<Signal> {
        assert!(self.inputs.contains(&signal.from));
        assert_eq!(self.name, signal.to);
        match signal.level {
            Level::Low => {
                self.flip();
                self.outputs.iter().map(|o| Signal::new(&self.name, &o, self.level)).collect()
            },
            Level::High => {
                Vec::new()
            },
        }
    }

    fn register_input(&mut self, name: &String) {
        self.inputs.push(name.clone());
    }

    fn register_output(&mut self, name: &String) {
        self.outputs.push(name.clone());
    }

    fn reset(&mut self) {
        self.level = Level::Low;
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct Conjunction {
    name: String,
    inputs: HashMap<String, Level>,
    outputs: Vec<String>,
}

impl Conjunction {
    fn new(name: &String) -> Self {
        Self{name: name.clone(), inputs: HashMap::new(), outputs: Vec::new()}
    }
}

impl Module for Conjunction {
    fn activate(&mut self, signal: Signal) -> Vec<Signal> {
        assert!(self.inputs.contains_key(&signal.from));
        assert_eq!(self.name, signal.to);
        *self.inputs.get_mut(&signal.from).unwrap() = signal.level;
        let all_inputs_high = self.inputs.values().all(|&l| matches!(l, Level::High));
        self.outputs.iter().map(|o| {
            Signal::new(&self.name, &o, match all_inputs_high {
                true => Level::Low,
                false => Level::High,
            })
        }).collect()
    }

    fn register_input(&mut self, name: &String) {
        self.inputs.insert(name.clone(), Level::Low);
    }

    fn register_output(&mut self, name: &String) {
        self.outputs.push(name.clone());
    }

    fn reset(&mut self) {
        self.inputs.values_mut().for_each(|v| *v = Level::Low);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct Broadcaster {
    outputs: Vec<String>,
}

impl Broadcaster {
    fn new() -> Self {
        Self{outputs: Vec::new()}
    }
}

impl Module for Broadcaster {
    fn activate(&mut self, signal: Signal) -> Vec<Signal> {
        self.outputs.iter().map(|o| Signal::new(&String::from("broadcaster"), &o, signal.level)).collect()
    }

    fn register_input(&mut self, _: &String) {
    }

    fn register_output(&mut self, name: &String) {
        self.outputs.push(name.clone());
    }

    fn reset(&mut self) {
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct Sink;

impl Module for Sink {
    fn activate(&mut self, _: Signal) -> Vec<Signal> {
        Vec::new()
    }

    fn register_input(&mut self, _: &String) {
    }

    fn register_output(&mut self, _: &String) {
    }

    fn reset(&mut self) {
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

type Graph = HashMap<String, Box<dyn Module>>;

fn build_graph(data: &String) -> Graph {
    let mut graph = Graph::new();
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for line in data.lines() {
        let (module, outputs) = line.split_once(" -> ").unwrap();
        let (mod_type, mod_name) = match module {
            "broadcaster" => ("b", "broadcaster"),
            _ => (module.get(0..1).unwrap(), module.get(1..).unwrap())
        };
        let mod_name = mod_name.to_string();
        match mod_type {
            "%" => graph.insert(mod_name.clone(), Box::new(FlipFlop::new(&mod_name))),
            "&" => graph.insert(mod_name.clone(), Box::new(Conjunction::new(&mod_name))),
            "b" => graph.insert(String::from("broadcaster"), Box::new(Broadcaster::new())),
            _ => unreachable!(),
        };

        for out_name in outputs.split(", ").map(|o| String::from(o)) {
            graph.get_mut(&mod_name).unwrap().register_output(&out_name);
            inputs.entry(out_name).or_insert(Vec::new()).push(mod_name.clone());
        }
    }

    for (mod_name, input_names) in inputs.iter() {
        let module = graph.entry(mod_name.clone()).or_insert(Box::new(Sink{}));
        for input_name in input_names {
            module.register_input(input_name);
        }
    }

    graph
}

fn solve_part_1(modules: &mut Graph) -> u64 {
    let mut signals = VecDeque::<Signal>::new();
    let mut low_signals = 0u64;
    let mut high_signals = 0u64;

    for _ in 0..1000 {
        signals.push_front(Signal::new(&String::from("button"), &String::from("broadcaster"), Level::Low));

        while let Some(signal) = signals.pop_front() {

            match signal.level {
                Level::Low => low_signals += 1,
                Level::High => high_signals += 1,
            };

            let module = modules.get_mut(&signal.to).unwrap();
            signals.extend(module.activate(signal));
        }
    }

    low_signals * high_signals
}

fn gcd(a: u64, b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    let mut m = a;
    let mut n = b;
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }

    n
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn solve_part_2(modules: &mut Graph) -> u64 {
    let mut signals:VecDeque<Signal> = VecDeque::new();

    let dyn_module = match modules.get_mut(&String::from("bn")) {
        Some(module) => module,
        None => return 0
    };

    let bn_module = dyn_module.as_mut().as_any().downcast_mut::<Conjunction>().unwrap();
    let num_bn_inputs = bn_module.inputs.len();
    let mut first_high: HashMap<String, u64> = HashMap::new();
    let mut iteration = 0u64;

    loop {
        iteration += 1;

        signals.push_front(Signal::new(&String::from("button"), &String::from("broadcaster"), Level::Low));

        while let Some(signal) = signals.pop_front() {
            if signal.to.as_str() == "bn" && matches!(signal.level, Level::High) {
                first_high.entry(signal.from.clone()).or_insert(iteration);
            }
            if first_high.len() == num_bn_inputs {
                let values: Vec<u64> = first_high.values().map(|v| *v).collect();
                return values.into_iter().reduce(|acc, elem| lcm(acc, elem)).unwrap();
            }
            let module = modules.get_mut(&signal.to).unwrap();
            signals.extend(module.activate(signal));
        }
    }
}

fn process_input(input_file_path: &str) -> (u64, u64) {
    let data = std::fs::read_to_string(input_file_path).unwrap();
    let mut modules = build_graph(&data);

    let part_1 = solve_part_1(&mut modules);

    modules.values_mut().for_each(|m| m.reset());

    let part_2 = solve_part_2(&mut modules);

    (part_1, part_2)
}

fn main() {
    println!("{:?}", process_input("day20/input/test_a.txt"));
    println!("{:?}", process_input("day20/input/test_b.txt"));
    println!("{:?}", process_input("day20/input/input.txt"));
}

