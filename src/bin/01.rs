use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (left, right) = parse_input(reader);
        let mut answer: usize = 0;
        for idx in 0..left.len() {
            answer += left[idx].abs_diff(right[idx]);
        }
        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left, right) = parse_input(reader);
        let mut answer: usize = 0;
        let counts = right.iter().counts();
        for idx in 0..left.len() {
            // If right doesn't contain left, then just skip it entirely.
            if right.contains(&left[idx]) {
                answer += left[idx] * counts[&left[idx]];
            }
        }
        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for line in reader.lines() {
        if let Some((a, b)) = line.expect(
            "this line should be consistent based on input requirements"
        ).split_once("   ") {
            left.push(a.parse::<usize>().unwrap());
            right.push(b.parse::<usize>().unwrap());
        }
    }
    left.sort();
    right.sort();
    (left, right)
}