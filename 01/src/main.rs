use std::io::{ self, Read };

fn main() {
    let mut input: String = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1 answer> {}", part1(&input));
    println!("Part 2 answer> {}", part2(&input));
}

fn get_sorted_totals(input: &str) -> Vec<u32> {
    let mut curr_total: u32 = 0;
    let mut result: Vec<u32> = vec![];
    for line in input.lines() {
        match line.parse::<u32>() {
            // If it's an integer, add it to the total
            Ok(n) => {
                curr_total += n;
            }
            // Otherwise push the total to the result and clear total
            Err(_n) => {
                result.push(curr_total);
                curr_total = 0;
            }
        }
    }

    result.push(curr_total);
    result.sort();
    return result;
}

fn part1(input: &str) -> u32 {
    let sorted_totals: Vec<u32> = get_sorted_totals(&input);

    return sorted_totals[sorted_totals.len() - 1];
}

fn part2(input: &str) -> u32 {
    let mut sorted_totals: Vec<u32> = get_sorted_totals(&input);
    let toptotal = sorted_totals.drain(sorted_totals.len() - 3..);
    return toptotal.sum();
}

mod testing {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"),
            24000
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("5000\n\n24000\n\n8000\n\n10000\n\n8900\n\n11000"), 45000);
    }
}