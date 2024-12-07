use regex::Regex;

use crate::common::read_task;

pub fn lvl03() {
    let input = read_task(3);
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for cap in re.captures_iter(input.as_str()) {
        if cap[0].parse::<String>().unwrap() == "do()" {
            enabled = true;
        } else if cap[0].parse::<String>().unwrap() == "don't()" {
            enabled = false;
        } else if enabled {
            result += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        }
    }
    println!("Result: {}", result);
}