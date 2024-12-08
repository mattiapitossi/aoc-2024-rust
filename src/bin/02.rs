use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let count = reader.lines()
            .filter_map(Result::ok)
            .map(|line| line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .filter(|vect_int| check_if_safe(vect_int))
            .count();
        Ok(count)
    }

    fn is_either_increasing_or_decreasing(vect: &[u64]) -> bool {
        vect.windows(2).all(|w| w[0] <= w[1]) || vect.windows(2).all(|w| w[0] >= w[1])
    }

    fn check_if_safe(vect: &[u64]) -> bool {
        vect.windows(2).all(|w| (1..=3).contains(&w[0].abs_diff(w[1]))) && is_either_increasing_or_decreasing(vect)
    }


    fn check_if_safe_ignore_one(vect: &[u64]) -> bool {
        let vec = vect.to_vec();
        if check_if_safe(&vec) {
            return true;
        } else {
            for i in 0..vec.len() {
                let mut vec = vect.to_vec();
                vec.remove(i);
                if check_if_safe(&vec) {
                    return true;
                }
            }
        }
        false
    }


    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let count = reader.lines()
            .filter_map(Result::ok)
            .map(|line| line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .filter(|vect_int| check_if_safe_ignore_one(vect_int))
            .count();
        Ok(count)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
