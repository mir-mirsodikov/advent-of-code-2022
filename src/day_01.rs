///
/// Advent of Code Day 1 : Calorie Counting
///
/// The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc.
/// that they've brought with them, one item per line.
/// Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
///
/// https://adventofcode.com/2022/day/1
use crate::utils::{get_file_for_day, print_results};

/// Responsible for handling the logic for day 1
/// Depending on the part, it will call the appropriate function
/// and print the result
pub fn handle_day_01(part: u8) {
    let result = match part {
        1 => day_01_part_1(),
        2 => day_01_part_2(),
        _ => 0,
    };

    print_results(1, part, result.to_string().as_str());
}

/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn day_01_part_1() -> u32 {
    let contents = get_file_for_day(1);

    let mut sum: u32 = 0;
    let mut max: u32 = 0;

    contents.split('\n').for_each(|line| {
        if line.is_empty() {
            max = if sum > max { sum } else { max };
            sum = 0;
            return;
        }

        sum += line.parse().unwrap_or(0);
    });

    max
}

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
fn day_01_part_2() -> u32 {
    let contents = get_file_for_day(1);

    let mut sum = 0;
    let mut nums = vec![0, 0, 0];

    contents.split('\n').for_each(|line| {
        if line.is_empty() {
            if let Some(i) = get_insertion_position(&nums, sum) {
                nums[i] = sum;
                nums.sort();
            }
            sum = 0;
            return;
        }

        sum += line.parse().unwrap_or(0);
    });

    let result = nums.iter().sum::<i32>();

    result as u32
}

/// Returns the index or `None` of the position where the number should be inserted
/// The list should be in descending order
fn get_insertion_position(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos: Option<usize> = Option::None;

    for (i, item) in nums.iter().enumerate() {
        if num > *item {
            pos = Option::Some(i);
            break;
        }
    }

    pos
}

#[cfg(test)]
mod tests {
    const DAY_01_RESULT_PART_1: u32 = 68467;
    const DAY_01_RESULT_PART_2: u32 = 203420;

    #[test]
    fn test_day_01_part_1() {
        let result = super::day_01_part_1();

        assert_eq!(result, DAY_01_RESULT_PART_1);
    }

    #[test]
    fn test_day_01_part_2() {
        let result = super::day_01_part_2();

        assert_eq!(result, DAY_01_RESULT_PART_2);
    }

    #[test]
    fn test_get_insertion_position() {
        let nums = vec![3, 2, 1];

        let mut pos = super::get_insertion_position(&nums, 4);
        assert_eq!(pos, Option::Some(0));

        pos = super::get_insertion_position(&nums, 3);
        assert_eq!(pos, Option::Some(1));

        pos = super::get_insertion_position(&nums, 2);
        assert_eq!(pos, Option::Some(2));

        pos = super::get_insertion_position(&nums, 0);
        assert_eq!(pos, Option::None);
    }
}
