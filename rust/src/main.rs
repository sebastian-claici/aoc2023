use rust::day1;
use rust::day2;
use rust::day3;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u32,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => {
            println!("{}", day1::solve_a("./input/day1.in").unwrap());
            println!("{}", day1::solve_b("./input/day1.in").unwrap());
        },
        2 => {
            println!("{}", day2::solve_a("./input/day2.in").unwrap());
            println!("{}", day2::solve_b("./input/day2.in").unwrap());
        },
        3 => {
            println!("{}", day3::solve_a("./input/day3.in").unwrap());
            println!("{}", day3::solve_b("./input/day3.in").unwrap());
        }
        _ => todo!("Not implemented yet")
    }
}
