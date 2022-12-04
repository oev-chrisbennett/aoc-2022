use std::io::{ self, Read };

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1 answer> {}", part1(&input));
    println!("Part 2 answer> {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut count: u32 = 0;
    for line in input.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        let elves: Vec<Vec<u32>> = ranges
            .into_iter()
            .map(|range: &str| {
                let range_parts: Vec<&str> = range.split("-").collect();
                let first: u32 = range_parts[0].parse::<u32>().unwrap();
                let second: u32 = range_parts[1].parse::<u32>().unwrap();
                vec![first, second]
            })
            .collect();

        let elf_a: Vec<u32> = (elves[0][0]..elves[0][1] + 1).collect();
        let elf_b: Vec<u32> = (elves[1][0]..elves[1][1] + 1).collect();

        if elf_b.iter().all(|item: &u32| elf_a.contains(item)) {
            count += 1;
        } else if elf_a.iter().all(|item: &u32| elf_b.contains(item)) {
            count += 1;
        }

        // -- Debug --
        // println!(
        //     "{:?}\t{:?}\t{:?}\t{}",
        //     elf_a.iter().all(|item| elf_b.contains(item)),
        //     elf_b.iter().all(|item| elf_a.contains(item)),
        //     elves,
        //     count
        // );
    }
    count
}

fn part2(input: &str) -> u32 {
    let mut count: u32 = 0;
    for line in input.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        let elves: Vec<Vec<u32>> = ranges
            .into_iter()
            .map(|range: &str| {
                let range_parts: Vec<&str> = range.split("-").collect();
                let first: u32 = range_parts[0].parse::<u32>().unwrap();
                let second: u32 = range_parts[1].parse::<u32>().unwrap();
                vec![first, second]
            })
            .collect();

        let elf_a: Vec<u32> = (elves[0][0]..elves[0][1] + 1).collect();
        let elf_b: Vec<u32> = (elves[1][0]..elves[1][1] + 1).collect();

        if elf_b.iter().any(|item: &u32| elf_a.contains(item)) {
            count += 1;
        } else if elf_a.iter().any(|item: &u32| elf_b.contains(item)) {
            count += 1;
        }

        // -- Debug --
        // println!(
        //     "{:?}\t{:?}\t{:?}\t{}",
        //     elf_a.iter().all(|item| elf_b.contains(item)),
        //     elf_b.iter().all(|item| elf_a.contains(item)),
        //     elves,
        //     count
        // );
    }
    count
}

mod testing {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"), 4);
    }
}