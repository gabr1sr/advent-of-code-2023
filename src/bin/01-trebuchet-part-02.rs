use std::fs::File;
use std::io::prelude::*;

fn get_str_numbers(string: &str) -> Vec<u32> {
    let words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    let mut data = words
        .iter()
        .flat_map(|w| string.match_indices(w).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    data.sort_by(|(index1, _), (index2, _)| index1.cmp(index2));

    data.iter()
        .map(|&(_, v)| (words.iter().position(|&w| w == v).unwrap() % 10) as u32)
        .collect()
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
        .map(|s| get_str_numbers(s))
        .map(|v| sum_first_and_last(v))
        .sum();

    println!("{data}");
}

#[test]
fn trebuchet_part_two() {
    let input = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    let data: u32 = input
        .iter()
        .map(|s| get_str_numbers(s))
        .map(|v| sum_first_and_last(v))
        .sum();

    assert_eq!(data, 281);
}
