mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod utils;

use crate::{
    day_01::handle_day_01, day_02::handle_day_02, day_03::handle_day_03, day_04::handle_day_04,
};

fn main() {
    println!("Advent of Code 2022");

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Please provide a day number");
        return;
    }

    let day = args[1].parse::<u8>().expect("Day number must be a number");
    let part = args[2].parse::<u8>().expect("Part number must be a number");

    if day > 25 {
        println!("Day number must be between 1 and 25");
        return;
    }

    if part > 2 {
        println!("Part number must be between 1 and 2");
        return;
    }

    match day {
        1 => handle_day_01(part),
        2 => handle_day_02(part),
        3 => handle_day_03(part),
        4 => handle_day_04(part),
        _ => println!("Day {} not implemented yet", day),
    };
}
