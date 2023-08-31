/// 
/// Advent of Code Day 2 : Rock Paper Scissors
/// 
/// The Elves begin to set up camp on the beach. 
/// To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
/// 
/// Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) 
/// that they say will be sure to help you win. 
/// "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" 
/// Suddenly, the Elf is called away to help with someone's tent.
/// 
/// The second column, you reason, must be what you should play in response: 
/// X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, 
/// so the responses must have been carefully chosen.
/// 
/// The winner of the whole tournament is the player with the highest score. 
/// Your total score is the sum of your scores for each round. 
/// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) 
/// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
/// 
/// Since you can't be sure if the Elf is trying to help you or trick you, 
/// you should calculate the score you would get if you were to follow the strategy guide.
/// 
/// https://adventofcode.com/2022/day/2


use crate::utils::{get_file_for_day, print_results};

pub fn handle_day_02(part: u8) {
    let result = match part {
        1 => part_1(),
        _ => part_2(),
    };

    print_results(2, part, result.to_string().as_str());
}

#[derive(Debug, PartialEq)]
enum GameOutcome {
    Win,
    Lose,
    Tie,
}

impl GameOutcome {
    /// Returns the score for the game outcome
    fn get_score(&self) -> u32 {
        match self {
            GameOutcome::Win => 6,
            GameOutcome::Tie => 3,
            GameOutcome::Lose => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum PlayersHand {
    Rock,
    Paper,
    Scissors,
}

impl PlayersHand {
    /// Returns the score for the players hand
    fn get_score(&self) -> u32 {
        match self {
            PlayersHand::Rock => 1,
            PlayersHand::Paper => 2,
            PlayersHand::Scissors => 3,
        }
    }
}

/// Returns the players hand for player 1
/// as a [PlayersHand] enum
fn get_player_1_hand(player_1: String) -> PlayersHand {
    match player_1.as_str() {
        "A" => PlayersHand::Rock,
        "B" => PlayersHand::Paper,
        "C" => PlayersHand::Scissors,
        _ => panic!("Invalid option"),
    }
}

/// Plays a game of rock paper scissors
/// and returns the outcome and score
fn play_game(player_1: String, player_2: String) -> u32 {
    let player_1 = get_player_1_hand(player_1);

    let player_2 = match player_2.as_str() {
        "X" => PlayersHand::Rock,
        "Y" => PlayersHand::Paper,
        "Z" => PlayersHand::Scissors,
        _ => panic!("Invalid option"),
    };

    let outcome = match (&player_2, player_1) {
        (PlayersHand::Rock, PlayersHand::Rock) => GameOutcome::Tie,
        (PlayersHand::Rock, PlayersHand::Paper) => GameOutcome::Lose,
        (PlayersHand::Rock, PlayersHand::Scissors) => GameOutcome::Win,
        (PlayersHand::Paper, PlayersHand::Rock) => GameOutcome::Win,
        (PlayersHand::Paper, PlayersHand::Paper) => GameOutcome::Tie,
        (PlayersHand::Paper, PlayersHand::Scissors) => GameOutcome::Lose,
        (PlayersHand::Scissors, PlayersHand::Rock) => GameOutcome::Lose,
        (PlayersHand::Scissors, PlayersHand::Paper) => GameOutcome::Win,
        (PlayersHand::Scissors, PlayersHand::Scissors) => GameOutcome::Tie,
    };

    let score = outcome.get_score() + player_2.get_score();

    score
}

fn get_score_for_desired_outcome(outcome: String, player_1: String) -> u32 {
    let desired_outcome = match outcome.as_str() {
        "X" => GameOutcome::Lose,
        "Y" => GameOutcome::Tie,
        "Z" => GameOutcome::Win,
        _ => panic!("Invalid outcome"),
    };

    let player_1 = get_player_1_hand(player_1);

    let player_2 = match (&desired_outcome, player_1) {
        (GameOutcome::Win, PlayersHand::Rock) => PlayersHand::Paper,
        (GameOutcome::Win, PlayersHand::Paper) => PlayersHand::Scissors,
        (GameOutcome::Win, PlayersHand::Scissors) => PlayersHand::Rock,
        (GameOutcome::Tie, PlayersHand::Rock) => PlayersHand::Rock,
        (GameOutcome::Tie, PlayersHand::Paper) => PlayersHand::Paper,
        (GameOutcome::Tie, PlayersHand::Scissors) => PlayersHand::Scissors,
        (GameOutcome::Lose, PlayersHand::Rock) => PlayersHand::Scissors,
        (GameOutcome::Lose, PlayersHand::Paper) => PlayersHand::Rock,
        (GameOutcome::Lose, PlayersHand::Scissors) => PlayersHand::Paper,
    };

    let score = desired_outcome.get_score() + player_2.get_score();

    score
}

fn part_1() -> u32 {
    let contents = get_file_for_day(2);

    let mut score = 0;

    contents.split("\n").into_iter().for_each(|line| {
        let players = line.split(" ").collect::<Vec<&str>>();

        let result = play_game(players[0].to_string(), players[1].to_string());

        score += result;
    });

    score
}

fn part_2() -> u32 {
    let contents = get_file_for_day(2);

    let mut score = 0;

    contents.split("\n").for_each(|line| {
        let players = line.split(" ").collect::<Vec<&str>>();

        let player_1 = players[0].to_string();
        let desired_outcome = players[1].to_string();

        score += get_score_for_desired_outcome(desired_outcome, player_1);
    });

    score
}

#[cfg(test)]
mod tests {
    const PART_1_RESULT: u32 = 11449;
    const PART_2_RESULT: u32 = 13187;

    #[test]
    fn test_part_1() {
        let result = super::part_1();

        assert_eq!(result, PART_1_RESULT);
    }

    #[test]
    fn test_part_2() {
        let result = super::part_2();

        assert_eq!(result, PART_2_RESULT);
    }

    #[test]
    fn test_play_game() {
        let result = super::play_game("A".to_string(), "Y".to_string());
        assert_eq!(result, 8);

        let result = super::play_game("B".to_string(), "X".to_string());
        assert_eq!(result, 1);

        let result = super::play_game("C".to_string(), "Z".to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn test_get_score_for_desired_outcome() {
        let result = super::get_score_for_desired_outcome("Y".to_string(), "A".to_string());
        assert_eq!(result, 4);

        let result = super::get_score_for_desired_outcome("X".to_string(), "B".to_string());
        assert_eq!(result, 1);

        let result = super::get_score_for_desired_outcome("Z".to_string(), "C".to_string());
        assert_eq!(result, 7);
    }
}
