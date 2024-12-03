use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_file(filename: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        
        if nums.len() == 2 {
            left_list.push(nums[0]);
            right_list.push(nums[1]);
        }
    }

    Ok((left_list, right_list))
}

fn calculate_total_distance(left_list: &[i32], right_list: &[i32]) -> i32 {
    // Create sorted copies of the input lists
    let mut left_sorted = left_list.to_vec();
    let mut right_sorted = right_list.to_vec();
    
    // Sort the lists
    left_sorted.sort_unstable();
    right_sorted.sort_unstable();
    
    // Calculate total distance
    left_sorted.iter()
        .zip(right_sorted.iter())
        .map(|(left, right)| (left - right).abs())
        .sum()
}

fn calculate_part2(left_list: &[i32], right_list: &[i32]) -> i32 {
    // Count occurrences of each number in right_list
    let mut right_counts = HashMap::new();
    for &num in right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the sum of left elements multiplied by their occurrences in right_list
    left_list.iter()
        .map(|&left| left * right_counts.get(&left).unwrap_or(&0))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the actual puzzle input
    let (puzzle_left, puzzle_right) = read_input_file("inputs/input_day01.txt")?;
    
    // Calculate and print part 1 and part 2 results
    println!(
        "Puzzle total distance: {}",
        calculate_total_distance(&puzzle_left, &puzzle_right)
    );

    println!(
        "Puzzle part2 result: {}",
        calculate_part2(&puzzle_left, &puzzle_right)
    );

    Ok(())
}
