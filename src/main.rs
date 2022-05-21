mod state;

use state::State;

use std::fmt::Write;
use std::fs;
use std::collections::BinaryHeap;

use clap::Parser;

/// Apportionment calculator based on the Huntington-Hill method
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[clap(short, long, default_value = "./data/us2020.tsv")]
    input: String,

    /// Output file
    #[clap(short, long)]
    output: Option<String>,

    /// Number of seats to apportion
    #[clap(short, long, default_value_t = 435)]
    seats: u32,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();
    let input_file_path = args.input;
    let mut seats_remaining = args.seats;

    // Initialize priority queue based on input file
    let mut queue = BinaryHeap::new();
    for line in fs::read_to_string(input_file_path).expect("Something went wrong reading the input file.").lines() {
        let pair = line.split("\t").collect::<Vec<&str>>();
        let name: String = pair[0].parse().unwrap();
        let population = pair[1].parse().unwrap();
        queue.push(State::new(name, population));
    }

    // Handle case where the number of seats is less than the number of states
    if queue.len() as u32 > seats_remaining {
        panic!("You must specify at least {} seats so that each state gets at least one seat.", queue.len());
    }
    seats_remaining -= queue.len() as u32;

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
    for state in states_list {
        write!(&mut output, "{}\t{}\t{}\t{:.2}\n", state.get_name(), state.get_population(), state.get_seats(), state.get_people_per_seat())
            .expect("Something went wrong generating the output.");
    }

    // Write to the output file, or if none is specified, print to the console
    if let Some(output_file_path) = args.output {
        fs::write(output_file_path, output).expect("Something went wrong writing to the output file.");
    } else {
        print!("{}", output);
    }
}