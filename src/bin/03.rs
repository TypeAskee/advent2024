use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let in_stuff = parse_input(reader);
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let mut answer: i32 = 0;
        for line in in_stuff {
            for m in re.find_iter(&*line) {
                let x = m.as_str().replace(r"mul(", "").replace(r")", "");
                let tmp: Vec<_> = x.split(",").collect();
                answer += &tmp[0].parse::<i32>().unwrap() * &tmp[1].parse::<i32>().unwrap();
            }
        }
        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let in_stuff = parse_input(reader);
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let mut answer: i32 = 0;
        for line in in_stuff {
            let clean_str = Regex::new(r"don't\(\)[\s\S]*?(do\(\)|$)").unwrap().replace_all(&*line, "").to_string();
            for m in re.find_iter(&*clean_str) {
                let x = m.as_str().replace(r"mul(", "").replace(r")", "");
                let tmp: Vec<_> = x.split(",").collect();
                answer += &tmp[0].parse::<i32>().unwrap() * &tmp[1].parse::<i32>().unwrap();
            }
        }
        Ok(answer)
    }
    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> Vec<String> {
    let mut left: Vec<String> = Vec::new();
    for line in reader.lines() {
        left.push(line.unwrap());
    }
    left.sort();
    left
}