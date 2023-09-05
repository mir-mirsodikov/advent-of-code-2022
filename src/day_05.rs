use crate::utils::{get_file_for_day, print_results};

pub fn handle_day_05(part: u8) {
    let result = match part {
        1 => part_1(),
        2 => part_2(),
        _ => "".to_owned(),
    };

    print_results(5, part, result.as_str());
}

fn create_initial_crate_stacks(setup: String) -> Vec<Vec<u32>> {
    let mut crate_stacks = Vec::new();

    let len = get_len_of_stacks(setup.split('\n').last().unwrap());

    setup.split('\n').rev().for_each(|line| {
        let clean_line = line.split(' ').collect::<Vec<&str>>().join(" ");
        println!("{}", clean_line);
    });

    for i in 0..3 {
        crate_stacks.push(Vec::new());
        crate_stacks[i].push(i as u32 + 1);
    }

    crate_stacks
}

fn get_len_of_stacks(first_line: &str) -> usize {
    first_line.split_whitespace().collect::<Vec<&str>>().len()
}

fn part_1() -> String {
    let file = get_file_for_day(5);

    let mut raw_crates: String = String::new();

    for line in file.split('\n') {
        if line.is_empty() {
            break;
        }
        raw_crates += line;
        raw_crates += "\n";
    }

    create_initial_crate_stacks(raw_crates.trim().to_owned());

    "".to_owned()
}

fn part_2() -> String {
    "".to_owned()
}


#[cfg(test)]
mod tests {
    
    #[test]
    fn test_get_len_of_stacks() {
        let result = super::get_len_of_stacks("1  2  3  4  5  6  7  8  9  10");
        assert_eq!(result, 10);

        let result = super::get_len_of_stacks("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15");
        assert_eq!(result, 15);
    }
}
