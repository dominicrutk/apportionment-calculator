mod state;

use state::State;

use std::env;
use std::fs;
use std::collections::BinaryHeap;

fn main() {
    // Defaults
    let mut file_path = "./data/2020us.txt";
    let mut seats_remaining = 435_u32;

    // User settings
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        file_path = &args[1];
    }
    if args.len() >= 3 {
        seats_remaining = args[2].parse::<u32>().expect("You must enter a nonnegative integer number of seats.");
    }

    let mut queue = BinaryHeap::new();
    for line in fs::read_to_string(file_path).expect("Something went wrong reading the file.").lines() {
        let pair = line.split("|").collect::<Vec<&str>>();
        let name: String = pair[0].parse().unwrap();
        let population = pair[1].parse().unwrap();
        queue.push(State::new(name, population));
    }

    if queue.len() as u32 > seats_remaining {
        panic!("You must specify at least {} seats so that each state gets at least one seat.", queue.len());
    }
    seats_remaining -= queue.len() as u32;

    while seats_remaining > 0 {
        let mut next_state = queue.pop().unwrap();
        next_state.add_seat();
        queue.push(next_state);
        seats_remaining -= 1;
    }

    let mut states_list = queue.into_vec();
    states_list.sort_by(|first, second| first.get_name().cmp(second.get_name()));

    println!("State|Population|Seats|People per Seat");
    for state in states_list {
        println!("{}|{}|{}|{}", state.get_name(), state.get_population(), state.get_seats(), state.get_people_per_seat());
    }
}