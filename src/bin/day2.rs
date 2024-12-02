use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(file_path: &str) -> Vec<Vec<i32>> {
    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    let levels: Vec<Vec<i32>> = buf
        .lines()
        .map(|line| {
            let level: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|report| report.parse().unwrap())
                .collect();
            return level;
        })
        .collect();
    return levels;
}

fn is_level_safe(level: &Vec<i32>) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    let mut steep_difference = false;
    for j in 1..level.len() {
        let report = level[j];
        let prev_report = level[j - 1];
        if prev_report < report {
            increasing = true;
        } else if prev_report > report {
            decreasing = true;
        }
        let abs_diff = (prev_report - report).abs();
        if abs_diff < 1 || abs_diff > 3 {
            steep_difference = true;
        }
        if !(increasing ^ decreasing) || steep_difference {
            return false;
        }
    }
    return true;
}

fn solve_part1(levels: &Vec<Vec<i32>>) -> i32 {
    levels
        .iter()
        .map(|level| is_level_safe(level) as i32)
        .sum::<i32>()
}

fn solve_part2(levels: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for level in levels {
        let mut removed_level = level.clone();
        for i in 0..level.len() {
            removed_level.remove(i);
            if is_level_safe(&removed_level) {
                count += 1;
                break;
            }
            removed_level.insert(i, level[i]);
        }
    }
    return count;
}

fn main() {
    let levels = parse_input("./inputs/day2.txt");
    let safe_levels = solve_part1(&levels);
    println!("# of safe levels: {}", safe_levels);
    let relaxed_safe_levels = solve_part2(levels);
    println!(
        "# of safe levels with problem dampner: {}",
        relaxed_safe_levels
    );
}
