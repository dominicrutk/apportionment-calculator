use std::env;
use std::fs;
use std::path::PathBuf;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    // Defaults
    let mut file: PathBuf = PathBuf::from("./data/2020us.txt");
    let mut seats: u32 = 435;

    // User settings
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        file = PathBuf::from(&args[1])
    }
    if args.len() >= 3 {
        seats = args[2].parse::<u32>().expect("You must enter a nonnegative integer number of seats.");
    }

    let mut queue = BinaryHeap::new();
    for line in fs::read_to_string(file).expect("Something went wrong reading the file.").lines() {
        let pair = line.split("|").collect::<Vec<&str>>();
        let name: String = pair[0].parse().unwrap();
        let population: u32 = pair[1].parse().unwrap();
        queue.push(State {
            name,
            population,
            seats: 1,
            priority: population as f64 / 2_f64.sqrt(),
        });
    }

    seats = seats - queue.len() as u32;
    if seats < 0 {
        panic!("There must be at least {} seats so that each state gets a seat.", queue.len());
    }

    while seats > 0 {
        let mut state = queue.pop().unwrap();
        // println!("{:?}", state);
        state.seats = state.seats + 1;
        state.priority = (state.population as f64) / ((state.seats * (state.seats + 1)) as f64).sqrt();
        queue.push(state);
        seats = seats - 1;
    }

    let mut vec = Vec::new();
    while let Some(State { name, seats, .. }) = queue.pop() {
        vec.push(OutputState {
            name,
            seats,
        });
    }

    vec.sort();

    for output in vec {
        println!("{}|{}", output.name, output.seats);
    }
}

#[derive(PartialEq, Debug)]
struct State {
    name: String,
    population: u32,
    seats: u32,
    priority: f64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.priority - other.priority {
            x if x > 0.0 => Some(Ordering::Greater),
            x if x < 0.0 => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.priority - other.priority {
            x if x > 0.0 => Ordering::Greater,
            x if x < 0.0 => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
struct OutputState {
    name: String,
    seats: u32,
}
