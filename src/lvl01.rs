use std::collections::HashMap;
use std::fs;

pub fn lvl01() {
    let task = read_task();
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in task.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(part1), Some(part2)) = (parts.next(), parts.next()) {
            if let (Ok(num1), Ok(num2)) = (part1.parse::<i32>(), part2.parse::<i32>()) {
                col1.push(num1);
                col2.push(num2);
            }
        }
    }
    col1.sort();
    col2.sort();
    let sum_of_differences: i32 = col1.iter().zip(&col2).map(|(a, b)| (a - b).abs()).sum();
    println!("Sum of differences: {}", sum_of_differences);
}

pub fn lvl01_second() {
    let task = read_task();
    let mut keys = Vec::new();
    let mut frequencies = HashMap::new();

    for line in task.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(part1), Some(part2)) = (parts.next(), parts.next()) {
            if let (Ok(num1), Ok(num2)) = (part1.parse::<i32>(), part2.parse::<i32>()) {
                keys.push(num1);
                *frequencies.entry(num2).or_insert(0) += 1;
            }
        }
    }
    let result: i32 = keys.iter().map(|key| frequencies.get(key).unwrap_or(&0) * key).sum();
    println!("Result: {}", result);
}

pub fn read_task() -> String {
    fs::read_to_string("tasks/lvl01.txt")
        .expect("input task is missing")
}