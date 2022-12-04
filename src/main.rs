use aoc_22::*;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Provide one argument");
        return;
    }

    match calculate(args[1].parse::<u32>().unwrap_or(0)) {
        Ok((a,b)) => {
            println!("Part 1: {}\nPart 2: {}", a, b);
        },
        Err(a) => {
            println!("{}", a);
        }
    };
}
