use std::error::Error;
use std::process;

use clap::Parser;
use colored::Colorize;
use fizzbuzz::fizzbuzz;

/// Simple fizzbuzz program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Maximum number to count up to. Only positive numbers accepted.
    #[arg(default_value_t = 10)]
    max: u32,
}

// takes ownership because Args is not meant to be used
// after that
fn run(config: Args) -> Result<(), Box<dyn Error>> {
    for i in 1..=config.max {
        let result = fizzbuzz(i);

        println!("{}", result.yellow().bold());
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(err) = run(args) {
        eprintln!("{}", format!("{err}").red());
        process::exit(-1);
    };
}
