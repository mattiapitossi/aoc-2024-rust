use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let mut left_column_list_as_str: Vec<u64> = lines.iter().map(|line| {
            let mut parts = line.split_whitespace();
            let left_column = parts.next().unwrap();
            left_column.to_string().parse().unwrap()
        }).collect();

        let mut right_column_list_as_str: Vec<u64> = lines.iter().map(|line| {
            let mut parts = line.split_whitespace();
            parts.next();
            let right_column = parts.next().unwrap();
            right_column.to_string().parse().unwrap()
        }).collect();

        left_column_list_as_str.sort();
        right_column_list_as_str.sort();

        let ordered_pairs = left_column_list_as_str.iter().zip(right_column_list_as_str.iter());

        //print ordered_pairs as list

        let res = ordered_pairs.map(|(left, right)| right.abs_diff(*left)).sum::<u64>();

        //Ok(res as usize)
        Ok(res as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}