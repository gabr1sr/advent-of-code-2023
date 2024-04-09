use std::fs::File;
use std::io::prelude::*;

fn get_str_digits(string: &str) -> Vec<u32> {
    string.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn sum_first_and_last(vector: Vec<u32>) -> u32 {
    let first = vector.first().unwrap() * 10;
    let last = vector.last().unwrap();
    first + last
}

fn main() {
    let args = std::env::args();
    let path = args.last().unwrap();
    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input: Vec<&str> = contents.trim().split("\n").collect();

    let data: u32 = input
        .iter()
        .map(|s| get_str_digits(s))
        .map(|v| sum_first_and_last(v))
        .sum();

    println!("{data}");
}

#[test]
fn trebuchet_part_one() {
    let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    let data: u32 = input
        .iter()
        .map(|s| get_str_digits(s))
        .map(|v| sum_first_and_last(v))
        .sum();

    assert_eq!(data, 142);
}
