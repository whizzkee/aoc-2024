use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_safe_report(levels: &[i32]) -> bool {
    // Check if levels are all increasing or all decreasing
    let increasing = levels.windows(2).all(|w| w[0] < w[1]);
    let decreasing = levels.windows(2).all(|w| w[0] > w[1]);

    // Check if differences are between 1 and 3
    let valid_diffs = levels.windows(2).all(|w| {
        let diff = (w[1] - w[0]).abs();
        diff >= 1 && diff <= 3
    });

    (increasing || decreasing) && valid_diffs
}

fn is_safe_with_problem_dampener(levels: &Vec<i32>) -> bool {
    // If the report is already safe, return true
    if is_safe_report(levels) {
        return true;
    }

    // Try removing each level once
    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);
        
        if is_safe_report(&modified_levels) {
            return true;
        }
    }

    false
}

fn main() -> std::io::Result<()> {
    // Read input file
    let file = File::open("inputs/input_day02.txt")?;
    let reader = BufReader::new(file);

    // Parse reports
    let reports: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| {
            line.ok().map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
        })
        .collect();

    // Part 1: Count safe reports
    let safe_reports_part1 = reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count();

    // Part 2: Count safe reports with Problem Dampener
    let safe_reports_part2 = reports
        .iter()
        .filter(|report| is_safe_with_problem_dampener(report))
        .count();

    println!("Part 1 - Number of safe reports: {}", safe_reports_part1);
    println!("Part 2 - Number of safe reports with Problem Dampener: {}", safe_reports_part2);

    Ok(())
}