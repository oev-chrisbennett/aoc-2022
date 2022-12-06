use std::{ fs };
use std::collections::HashSet;
pub fn day_06(strlen: usize) {
    let input = fs::read_to_string("input").unwrap();

    for (index, _ch) in input.chars().enumerate() {
        if index < input.len() - 1 {
            let vec_substr: &str = &input[index..index + &strlen];
            let mut hs = HashSet::new();
            for letter in vec_substr.chars() {
                hs.insert(letter);
            }

            if hs.len() == strlen {
                println!("{}{:?}", index + strlen, hs);
                break;
            }
        }
    }
}
pub fn part_one() {
    day_06(4)
}

pub fn part_two() {
    day_06(14)
}

pub fn main() {
    part_one();
    part_two();
}