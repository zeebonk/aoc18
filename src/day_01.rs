use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day_01.txt");

fn main() {
    println!("Part 1: {}", solve_part_1(INPUT));
    println!("Part 2: {}", solve_part_2(INPUT));
}

fn solve_part_1(input: &str) -> i32 {
    input
        .split(char::is_whitespace)
        .flat_map(|s| s.parse::<i32>())
        .sum()
}

fn solve_part_2(input: &str) -> i32 {
    let mut numbers =
        input
        .split(char::is_whitespace)
        .flat_map(|s| s.parse::<i32>())
        .cycle();

    let mut frequency = 0;
    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    while seen_frequencies.insert(frequency) {
        frequency += numbers.next().expect("Iterator ended unexpectedly");
    }
    return frequency;
}

#[cfg(test)]
mod day_1_part_1 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_part_1("+1 -2 +3 +1"),  3);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part_1("+1 +1 +1"),  3);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_part_1("+1 +1 -2"),  0);
    }

    #[test]
    fn example_4() {
        assert_eq!(solve_part_1("-1 -2 -3"),  -6);
    }
}

#[cfg(test)]
mod day_2_part_2 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_part_2("+1 -1"),  0);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part_2("+3 +3 +4 -2 -4"),  10);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_part_2("-6 +3 +8 +5 -6"),  5);
    }

    #[test]
    fn example_4() {
        assert_eq!(solve_part_2("+7 +7 -2 -7 -4"),  14);
    }
}
