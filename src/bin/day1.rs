use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

fn parse_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = fs::File::open(file_path).unwrap();
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| {
            l.map_or_else(
                |_| (0, 0),
                |s| {
                    let mut parts = s.split_whitespace();
                    let num1: i32 = parts.next_back().unwrap().parse().unwrap();
                    let num2: i32 = parts.next_back().unwrap().parse().unwrap();
                    (num2, num1)
                },
            )
        })
        .unzip()
}

fn solve_part1(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();
    list1
        .iter()
        .zip(list2.iter())
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum()
}

fn solve_part2(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for num in list2.iter() {
        let cur_value = counter.get(num).unwrap_or(&0);
        counter.insert(*num, *cur_value + 1);
    }

    let mut similarity = 0;

    for num in list1.iter() {
        similarity += *num * (*counter.get(num).unwrap_or(&0));
    }

    similarity
}

fn main() {
    let file_path = "./inputs/day1.txt";
    let (list1, list2) = parse_input(file_path);

    let difference = solve_part1(list1.clone(), list2.clone());
    println!("difference: {}", difference);
    let similarty = solve_part2(list1, list2);
    println!("similarity: {}", similarty);
}
