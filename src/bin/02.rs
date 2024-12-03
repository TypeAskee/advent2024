use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let coming_in = parse_input(reader);
        let mut answer: usize = 0;
        for idx in 0..coming_in.len() {
            if is_valid(&coming_in[idx]) {
                answer += 1;
            }
        }
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let coming_in = parse_input(reader);
        let mut answer: usize = 0;
        let mut testing: Vec<i32>;
        for idx in 0..coming_in.len() {
            if is_valid(&coming_in[idx]) {
                answer += 1;
            }
            else {
                for y in 0..coming_in[idx].len() {
                    testing = coming_in[idx].clone();
                    testing.remove(y);
                    if is_valid(&testing) {
                        answer += 1;
                        break;
                    }
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    let mut answers: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        answers.push(line.unwrap().split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect_vec());
    }
    answers
}

fn is_valid(in_vec: &[i32]) -> bool {
    let up: bool;
    let mut diff: i32;
    if in_vec[0] > in_vec[1] {
        up = false;
    } else { up = true; }
    for idx_b in 0..in_vec.len() - 1 {
        diff = in_vec[idx_b] - in_vec[idx_b + 1];
        if up && diff >= 0 {
            return false;
        }
        if !up && diff <= 0 {
            return false;
        }
        if diff.abs_diff(0) > 3 {
            return false;
        }
    }
    return true;
}