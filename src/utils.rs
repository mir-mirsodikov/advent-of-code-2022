use std::{fs::File, io::Read};

const BASE_PATH: &str = "inputs/";

pub fn get_file_for_day(day: u8) -> String {
    let path = format!(
        "{}day_{}.txt",
        BASE_PATH,
        pad_number_with_zeroes(day),
    );

    let mut file = File::open(path).expect("Input file could not be opened");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Could not read file contents");

    content
}

pub fn print_results(day: u8, part: u8, result: &str) {
    println!("Day {} Part {}: {}", day, part, result);
}

fn pad_number_with_zeroes(number: u8) -> String {
    if number < 10 {
        format!("0{}", number)
    } else {
        format!("{}", number)
    }
}
