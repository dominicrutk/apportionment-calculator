# Apportionment Calculator

This command line calculator can apportion seats based on the [Huntington-Hill method](https://en.wikipedia.org/wiki/Huntington%E2%80%93Hill_method)
(the method [used by the U.S. House](https://en.wikipedia.org/wiki/United_States_congressional_apportionment#The_method_of_equal_proportions)).
It was inspired by [this calculator](https://isr.umich.edu/apportionment-calculator-for-us-census/) from the University
of Michigan, but it is designed to be more flexible by (a) not being restricted to U.S. states and (b) not being limited
to 999 seats.

## Usage

Currently, you must have Rust installed to run this tool (instructions [here](https://www.rust-lang.org/tools/install)).
Hopefully, that will change in the future.

To run the calculator with 435 seats and the 2020 U.S. Census numbers, simply run:

```bash
cargo run
```

Of course, you could have Googled that information. You came here for customization.

To adjust the state names, add/remove states, or adjust populations, create a new text file. In it, each row should contain
the state name, a pipe character, and the state population without commas (e.g. `Alabama|5024279`). A full example of the
2020 U.S. Census is available in a file named `2020us.txt` under the `data` directory.

Once you have a file with state names and populations, simply pass it as an argument to the `cargo run` command.

```bash
cargo run path_to_file.txt
```

If you want to adjust the total number of seats, pass another argument after the file name. Note that you must specify a
file name as the first argument in order to specify the number of seats.

```bash
cargo run path_to_file.txt 1000
```

As always, you can pipe the output of the calculator to a file.

```bash
cargo run > output.txt
cargo run path_to_file.txt 1000 > output.txt
```
