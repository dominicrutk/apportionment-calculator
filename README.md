# Apportionment Calculator

This command line calculator can apportion seats based on the [Huntington-Hill method](https://en.wikipedia.org/wiki/Huntington%E2%80%93Hill_method)
(the method [used by the U.S. House](https://en.wikipedia.org/wiki/United_States_congressional_apportionment#The_method_of_equal_proportions)).
It was inspired by [this calculator](https://isr.umich.edu/apportionment-calculator-for-us-census/) from the University
of Michigan, but it is designed to be more flexible by (a) not being restricted to U.S. states and (b) not being limited
to 999 seats.

## Usage

Currently, you must have Rust installed to run this tool (instructions [here](https://www.rust-lang.org/tools/install)).

To run the calculator with 435 seats and the 2020 U.S. Census numbers, simply run:

```bash
cargo run
```

Of course, you could have Googled that information. You came here for customization.

To adjust the state names, add/remove states, or adjust populations, create a new tab-separated value (`tsv`) file. In it,
each row should contain the state name, a tab character, and the state population without commas. A full example of the
2020 U.S. Census is available in a file named `us2020.tsv` under the `data` directory.

Once you have a file with state names and populations, simply pass it as the `--input` (`-i`) argument to the `cargo run` command.

```bash
cargo run -- -i path_to_file.tsv
```

If you want to adjust the total number of seats, pass the desired number as the `--seats` (`-s`) argument.

```bash
cargo run -- -s 1000
```

Alternatively, use the `--cube-root-method` (`-c`) flag to calculate the number of seats using the
[cube root method](https://en.wikipedia.org/wiki/Cube_root_rule). Note that this flag will be overridden by any number
seats specified by the `--seats` argument.

```bash
cargo run -- -c
```

By default, the tab-separated results will appear in the command line.
If you want to specify an output file for the results, pass it as the `--output` (`-o`) argument.

```bash
cargo run -- -o path_to_file.tsv
```

You can specify these arguments in any order, and each argument is optional.
- The input file will default to the 2020 U.S. Census data (`data/us2020.tsv`).
- The number of seats will default to 435.
- The output will default to the command line.

## Data

The `data` directory currently contains all U.S. Census data since 1990.
Contributions of older data or data from other countries would be much appreciated. Simply submit a pull request.
