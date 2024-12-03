use regex::Regex;
use std::fs;

fn parse_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

fn solve_part1(contents: &String) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(contents)
        .map(|caps| {
            let (_, [num1, num2]) = caps.extract();
            num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap()
        })
        .sum()
}

fn solve_part2(contents: &String) -> i64 {
    let mut mult = true;
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(don't\(\))|(do\(\))").unwrap();
    re.captures_iter(contents)
        .map(|caps| {
            let (m, [grp]) = caps.extract();
            let mut res = 0;
            if m == "don't()" {
                mult = false;
            } else if m == "do()" {
                mult = true;
            } else if mult {
                let (num1, num2) = grp.split_once(',').unwrap();
                res = num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap()
            }
            res
        })
        .sum()
}
fn main() {
    let contents = parse_input("./inputs/day3.txt");
    let result = solve_part1(&contents);
    println!("multiplication result part 1: {}", result);
    let result = solve_part2(&contents);
    println!("multiplication result part 2: {}", result);
}
