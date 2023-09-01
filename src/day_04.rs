/// Advent of Code Day 4 : Camp Cleanup
/// https://adventofcode.com/2022/day/4
use crate::utils::{get_file_for_day, print_results};

pub fn handle_day_04(part: u8) {
    let result = match part {
        1 => part_1(),
        2 => part_2(),
        _ => 0,
    };

    print_results(4, part, result.to_string().as_str());
}

fn parse_input_line(line: &str) -> (u32, u32) {
    let lower_bound = line[0..line.find("-").unwrap()]
        .to_string()
        .parse::<u32>()
        .unwrap();

    let upper_bound = &line[line.find("-").unwrap() + 1..line.len()]
        .to_string()
        .parse::<u32>()
        .unwrap();

    (lower_bound, *upper_bound)
}

fn part_1() -> u32 {
    let file = get_file_for_day(4);

    let mut sum = 0;

    file.split("\n").for_each(|line| {
        let (first_elf, mut second_elf) = line.split_at(line.find(",").unwrap());
        second_elf = second_elf.trim_start_matches(",");

        let (first_elf_lower_bound, first_elf_upper_bound) = parse_input_line(first_elf);
        let (second_elf_lower_bound, second_elf_upper_bound) = parse_input_line(second_elf);

        if first_elf_lower_bound >= second_elf_lower_bound
            && first_elf_upper_bound <= second_elf_upper_bound
            || second_elf_lower_bound >= first_elf_lower_bound
                && second_elf_upper_bound <= first_elf_upper_bound
        {
            sum += 1;
        }
    });

    sum
}

fn part_2() -> u32 {
    let file = get_file_for_day(4);

    let mut sum = 0;

    file.split("\n").for_each(|line| {
        let (first_elf, mut second_elf) = line.split_at(line.find(",").unwrap());
        second_elf = second_elf.trim_start_matches(",");

        let (first_elf_lower_bound, first_elf_upper_bound) = parse_input_line(first_elf);
        let (second_elf_lower_bound, second_elf_upper_bound) = parse_input_line(second_elf);

        if first_elf_lower_bound <= second_elf_upper_bound
            && first_elf_upper_bound >= second_elf_lower_bound
            || second_elf_lower_bound <= first_elf_upper_bound
                && second_elf_upper_bound >= first_elf_lower_bound
        {
            sum += 1;
        }
    });

    sum
}

#[cfg(test)]
mod tests {
    const PART_1_RESULT: u32 = 524;
    const PART_2_RESULT: u32 = 798;

    #[test]
    fn test_part_1() {
        assert_eq!(super::part_1(), PART_1_RESULT);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part_2(), PART_2_RESULT);
    }

    #[test]
    fn test_parse_input_line() {
        assert_eq!(super::parse_input_line("1-3"), (1, 3));
        assert_eq!(super::parse_input_line("2-9"), (2, 9));
    }
}
