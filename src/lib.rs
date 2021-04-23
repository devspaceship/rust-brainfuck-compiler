use std::env;
use std::error::Error;
// use std::fs;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub run: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let run = match args.next() {
            Some(arg) => arg == "--run" || arg == "-r",
            None => false,
        };

        Ok(Self { filename, run })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Rust Brainfuck Compiler");
    println!("{:?}", config);
    Ok(())
}
