use std::{ io::{ self, Read }, collections::HashMap };

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1 answer> {}", part1(&input));
    println!("Part 2 answer> {}", part2(&input));
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

impl Move {
    fn value_of_moves(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Result {
    fn value_of_win(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

fn define_scores_part1() -> HashMap<&'static str, u32> {
    let game_score = HashMap::from([
        ("A X", Move::Rock.value_of_moves() + Result::Draw.value_of_win()),
        ("A Y", Move::Paper.value_of_moves() + Result::Win.value_of_win()),
        ("A Z", Move::Scissors.value_of_moves() + Result::Lose.value_of_win()),
        ("B X", Move::Rock.value_of_moves() + Result::Lose.value_of_win()),
        ("B Y", Move::Paper.value_of_moves() + Result::Draw.value_of_win()),
        ("B Z", Move::Scissors.value_of_moves() + Result::Win.value_of_win()),
        ("C X", Move::Rock.value_of_moves() + Result::Win.value_of_win()),
        ("C Y", Move::Paper.value_of_moves() + Result::Lose.value_of_win()),
        ("C Z", Move::Scissors.value_of_moves() + Result::Draw.value_of_win()),
    ]);

    return game_score;
}

fn define_scores_part2() -> HashMap<&'static str, u32> {
    let game_score = HashMap::from([
        ("A X", Move::Scissors.value_of_moves() + Result::Lose.value_of_win()),
        ("B X", Move::Rock.value_of_moves() + Result::Lose.value_of_win()),
        ("C X", Move::Paper.value_of_moves() + Result::Lose.value_of_win()),
        ("A Y", Move::Rock.value_of_moves() + Result::Draw.value_of_win()),
        ("B Y", Move::Paper.value_of_moves() + Result::Draw.value_of_win()),
        ("C Y", Move::Scissors.value_of_moves() + Result::Draw.value_of_win()),
        ("A Z", Move::Paper.value_of_moves() + Result::Win.value_of_win()),
        ("B Z", Move::Scissors.value_of_moves() + Result::Win.value_of_win()),
        ("C Z", Move::Rock.value_of_moves() + Result::Win.value_of_win()),
    ]);

    return game_score;
}

fn part1(input: &str) -> u32 {
    let score: HashMap<&str, u32> = define_scores_part1();
    let mut total: u32 = 0;
    for line in input.lines() {
        total += score[line];
    }

    total
}

fn part2(input: &str) -> u32 {
    let score: HashMap<&str, u32> = define_scores_part2();
    let mut total: u32 = 0;
    for line in input.lines() {
        total += score[line];
    }

    total
}

mod testing {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("A Y\nB X\nC Z"), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("A Y\nB X\nC Z"), 12);
    }
}