use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod day1;

pub trait Solver {
    fn solve(self, input: BufReader<File>) -> String;
}

fn main() -> Result<(), io::Error> {
    let day = args().nth(1).unwrap();

    let (part1, part2) = match day.as_str() {
        "1" => (day1::Part1().solve(get_input(&day)?), day1::Part2().solve(get_input(&day)?)),
        _ => (String::from("unimplemented"), String::from("unimplemented")),
    };

    println!("Day {}\nPart 1: {}\nPart 2: {}", day, part1, part2);

    Ok(())
}

fn get_input(day: &str) -> Result<BufReader<File>, std::io::Error> {
    let input = File::open(format!("inputs/{}.txt", day))?;
    Ok(BufReader::new(input))
}
