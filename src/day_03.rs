/// Advent of Code Day 3 : Rucksack Reorganization
/// https://adventofcode.com/2022/day/3
use std::collections::HashMap;

use crate::utils::{get_file_for_day, print_results};

pub fn handle_day_03(part: u8) {
    let result = match part {
        1 => part_1(),
        2 => part_2(),
        _ => 0,
    };

    print_results(3, part, result.to_string().as_str());
}

fn part_1() -> u32 {
    let file = get_file_for_day(3);

    let mut priorities: u32 = 0;

    file.split('\n').for_each(|line| {
        let (part1, part2) = line.split_at(line.len() / 2);

        let mut part_1_map = HashMap::new();

        part1.chars().for_each(|c| {
            part_1_map.insert(c, 1);
        });

        for i in 0..part2.len() {
            let c = part2.chars().nth(i).unwrap();
            if part_1_map.contains_key(&c) {
                if c.is_uppercase() {
                    priorities += u32::from(c as u8) - 38;
                } else {
                    priorities += u32::from(c as u8) - 96;
                }

                break;
            }
        }
    });

    priorities
}

fn part_2() -> u32 {
    let file = get_file_for_day(3);

    let mut priorities: u32 = 0;
    let mut i = 0;

    let mut first_map = HashMap::new();
    let mut second_map = HashMap::new();

    file.split('\n').for_each(|line| {
        match i % 3 {
            0 => {
                first_map.clear();
                line.chars().for_each(|c| {
                    first_map.insert(c, 1);
                });
            }
            1 => {
                second_map.clear();
                line.chars().for_each(|c| {
                    if first_map.contains_key(&c) {
                        second_map.insert(c, 1);
                    }
                });
            }
            2 => {
                for i in 0..line.len() {
                    let c = line.chars().nth(i).unwrap();
                    if second_map.contains_key(&c) {
                        if c.is_uppercase() {
                            priorities += u32::from(c as u8) - 38;
                        } else {
                            priorities += u32::from(c as u8) - 96;
                        }
                        break;
                    }
                }
            }
            _ => {}
        }

        i += 1;
    });

    priorities
}

#[cfg(test)]
mod tests {
    const PART_1_RESULT: u32 = 7831;
    const PART_2_RESULT: u32 = 2683;

    #[test]
    fn test_part_1() {
        assert_eq!(super::part_1(), PART_1_RESULT);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part_2(), PART_2_RESULT);
    }
}
