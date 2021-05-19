mod state;

use state::State;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::collections::BinaryHeap;

fn main() {
    // Defaults
    let mut file: PathBuf = PathBuf::from("./data/2020us.txt");
    let mut seats_remaining: u32 = 435;

    // User settings
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        file = PathBuf::from(&args[1])
    }
    if args.len() >= 3 {
        seats_remaining = args[2].parse::<u32>().expect("You must enter a nonnegative integer number of seats.");
    }

    let mut queue = BinaryHeap::new();
    for line in fs::read_to_string(file).expect("Something went wrong reading the file.").lines() {
        let pair = line.split("|").collect::<Vec<&str>>();
        let name: String = pair[0].parse().unwrap();
        let population: u32 = pair[1].parse().unwrap();
        queue.push(State::new(name, population));
    }

    seats_remaining = seats_remaining - queue.len() as u32;
    if seats_remaining < 0 {
        panic!("There must be at least {} seats so that each state gets a seat.", queue.len());
    }

    while seats_remaining > 0 {
        let mut next_state = queue.pop().unwrap();
        next_state.add_seat();
        queue.push(next_state);
        seats_remaining = seats_remaining - 1;
    }

    let mut vec = queue.into_vec();
    vec.sort_by(|first, second| first.get_name().cmp(second.get_name()));

    println!("State|Population|Seats|People per Seat");
    for state in vec {
        println!("{}|{}|{}|{}", state.get_name(), state.get_population(), state.get_seats(), state.get_people_per_seat());
    }
}