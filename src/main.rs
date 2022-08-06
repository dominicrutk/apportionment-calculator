mod state;

use state::State;

use std::fmt::Write;
use std::{cmp, fs};
use std::collections::BinaryHeap;

use clap::Parser;

/// Apportionment calculator based on the Huntington-Hill method
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[clap(short, long, default_value = "./data/us2020.tsv")]
    input: String,

    /// Output file [default: stdout]
    #[clap(short, long)]
    output: Option<String>,

    /// Number of seats to apportion [default: 435]
    #[clap(short, long)]
    seats: Option<u64>,

    /// Multiplier to use with the cube root rule (e.g. specify 2 to get a total number of seats
    /// equal to twice the cube root of the total population)
    #[clap(short, value_name = "MULTIPLIER")]
    cube_root_multiplier: Option<u64>,

    /// Whether to use the cube root rule (with a multiplier of 1) to determine the total number of seats
    #[clap(short = 'C')]
    cube_root_rule: bool,

    /// Number of seats to apportion to the smallest state using the Wyoming rule
    #[clap(short, value_name = "SEATS")]
    wyoming_seats: Option<u64>,

    /// Whether to use the Wyoming rule with 1 seat for the smallest state
    #[clap(short = 'W')]
    wyoming_rule: bool,
}

fn main() {
    // Parse input file path
    let args = Args::parse();
    let input_file_path = args.input;

    // Initialize priority queue based on input file and find total population
    let mut queue = BinaryHeap::new();
    let mut total_population = 0u64;
    let mut min_population = u64::MAX;
    for line in fs::read_to_string(input_file_path).expect("Something went wrong reading the input file.").lines() {
        let pair = line.split("\t").collect::<Vec<&str>>();
        let name: String = pair[0].parse().unwrap();
        let population = pair[1].parse().unwrap();
        queue.push(State::new(name, population));
        total_population += population;
        min_population = cmp::min(population, min_population);
    }

    // Determine the number of seats
    // If a specific number if specified, use that number
    // If not, use the Wyoming rule (with the specified number of seats for Wyoming), if specified
    // If not, use the cube root rule (with the specified multiplier), if specified
    // Otherwise, default to 435
    let seats = if let Some(seats) = args.seats {
        seats
    } else if let Some(seats) = args.wyoming_seats {
        ((total_population as f64 / min_population as f64) * seats as f64).round() as u64
    } else if args.wyoming_rule {
        (total_population as f64 / min_population as f64).round() as u64
    } else if let Some(multiplier) = args.cube_root_multiplier {
        ((total_population as f64).powf(1f64 / 3f64) * multiplier as f64).round() as u64
    } else if args.cube_root_rule {
        (total_population as f64).powf(1f64 / 3f64).round() as u64
    } else {
        435
    };
    println!("Attempting to apportion {} seats...", seats);

    // Handle case where the number of seats is less than the number of states
    let mut seats_remaining = seats;
    if queue.len() as u64 > seats_remaining {
        panic!("You must specify at least {} seats so that each state gets at least one seat.", queue.len());
    }
    seats_remaining -= queue.len() as u64;

    // Apportion all remaining seats using the priority queue
    while seats_remaining > 0 {
        let mut next_state = queue.pop().unwrap();
        next_state.add_seat();
        queue.push(next_state);
        seats_remaining -= 1;
    }

    // Sort the states alphabetically into a list
    let mut states_list = queue.into_vec();
    states_list.sort_by(|first, second| first.get_name().cmp(second.get_name()));

    // Generate the output string
    let mut output = String::from("State\tPopulation\tSeats\tPeople Per Seat\n");
    for state in &states_list {
        write!(&mut output, "{}\t{}\t{}\t{:.4}\n", state.get_name(), state.get_population(), state.get_seats(), state.get_people_per_seat())
            .expect("Something went wrong generating the output.");
    }

    // Write to the output file, or if none is specified, print to the console
    if let Some(output_file_path) = args.output {
        fs::write(output_file_path, output).expect("Something went wrong writing to the output file.");
    } else {
        print!("{}", output);
    }

    println!("Successfully apportioned {} seats among {} states!", seats, states_list.len());
}