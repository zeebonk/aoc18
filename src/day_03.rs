extern crate regex;

mod shared;

use regex::Regex;
use shared::output_and_time;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day_03.txt");

fn main() {
    output_and_time("Part 1", || Box::new(solve_part_1(INPUT)));
    output_and_time("Part 2", || Box::new(solve_part_2(INPUT)));
}

fn solve_part_1(input: &str) -> usize {
    let re = Regex::new(r"(?m)#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut grid = [[0u8; 1000]; 1000];
    for cap in re.captures_iter(input) {
        let left: usize = cap[2].parse().unwrap();
        let top: usize = cap[3].parse().unwrap();
        let width: usize = cap[4].parse().unwrap();
        let height: usize = cap[5].parse().unwrap();

        for y in top..top + height {
            for x in left..left + width {
                grid[y][x] += 1u8;
            }
        }
    }

    grid
        .iter()
        .map(|row| row.iter().filter(|field| *field >= &2u8).count())
        .sum()
}

fn solve_part_2(input: &str) -> u16 {
    let re = Regex::new(r"(?m)#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut no_overlap_ids = HashSet::new();

    let mut grid = [[0u16; 1000]; 1000];
    for cap in re.captures_iter(input) {
        let id: u16 = cap[1].parse().unwrap();
        let left: usize = cap[2].parse().unwrap();
        let top: usize = cap[3].parse().unwrap();
        let width: usize = cap[4].parse().unwrap();
        let height: usize = cap[5].parse().unwrap();

        let mut overlap = false;
        for y in top..top + height {
            for x in left..left + width {
                overlap |= grid[y][x] != 0;
                no_overlap_ids.remove(&grid[y][x]);
                grid[y][x] = id;
            }
        }
        if !overlap {
            no_overlap_ids.insert(id);
        }
    }

    *no_overlap_ids.iter().next().unwrap()
}

#[cfg(test)]
mod day_3_part_1 {
    use super::solve_part_1;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_part_1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"),
            4
        );
    }
}

#[cfg(test)]
mod day_3_part_2 {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_part_2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"),
            3
        );
    }
}
