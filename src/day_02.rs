use std::collections::{HashSet, HashMap};

const INPUT: &str = include_str!("../input/day_02.txt");

fn main() {
    let input = INPUT.trim();
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input).unwrap());
}

fn solve_part_1(input: &str) -> i32 {
    let mut n_doubles = 0;
    let mut n_triples = 0;

    for line in input.split(char::is_whitespace) {
        let counts = char_counts(line);
        if counts.contains(&2) { n_doubles += 1 }
        if counts.contains(&3) { n_triples += 1 }
    }

    n_doubles * n_triples
}

fn char_counts(input: &str) -> HashSet<i32> {
    input
    .chars()
    .fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
    .iter()
    .map(|(_, cnt)| *cnt)
    .collect()
}

fn solve_part_2(input: &str) -> Option<String> {
    let lines: Vec<&str> =
        input
        .split(char::is_whitespace)
        .collect();

    for line_a in lines.iter() {
        for line_b in lines.iter() {
            let common: String =
                line_a
                .chars()
                .zip(line_b.chars())
                .filter(|(c1, c2)| c1 == c2)
                .map(|(c1, _)| c1)
                .collect();

            if common.len() == line_a.len() - 1 {
                return Some(common)
            }
        }
    }

    return None;
}

#[cfg(test)]
mod day_2_part_1 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_part_1("abcdef bababc abbcde abcccd aabcdd abcdee ababab"),
            12
        );
    }
}

#[cfg(test)]
mod day_2_part_2 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_part_2("abcde fghij klmno pqrst fguij axcye wvxyz"),
            Some(String::from("fgij"))
        );
    }
}
