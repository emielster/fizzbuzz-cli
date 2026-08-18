use std::env;
use std::error::Error;
use std::process;

use colored::Colorize;

use fizzbuzz::fizzbuzz;

#[derive(Debug)]
struct Config {
    max: u32,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let max: u32 = args
            .get(1)
            .ok_or("failed to get first argument. does it exist?")?
            .parse()?;

        Ok(Config { max })
    }
}

// takes ownership because Config is not meant to be used
// after that
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for i in 1..=config.max {
        let result = fizzbuzz(i);

        println!("{}", result.as_str().yellow().bold());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{}", format!("{err}").red()); // needs some special format
        eprintln!("{}", "usage: fizzbuzz max_iterations".green().bold());
        process::exit(-1);
    });

    if let Err(err) = run(config) {
        eprintln!("{}", format!("{err}").red());
        process::exit(-1);
    };
}
