use aoc_22::*;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Provide one argument");
        return;
    }

    println!("{:?}", calculate(args[1].parse::<u32>().unwrap_or(0)));
}
