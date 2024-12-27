use std::collections::{HashMap, HashSet};

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl22.txt");
    result = input
        .lines()
        .map(|line| {
            let mut number = line.parse::<u64>().unwrap();
            for _ in 0..2000 {
                number = get_next(number);
            }
            println!("{} -> {}", line, number);
            number
        })
        .sum();

    println!("Result = {}", result);
}

fn get_next(number: u64) -> u64 {
    let mut step1 = number * 64;
    step1 ^= number;
    step1 %= 16777216;
    let mut step2 = step1 / 32;
    step2 ^= step1;
    step2 %= 16777216;
    let mut result = step2 * 2048;
    result ^= step2;
    result % 16777216
}

fn part2() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl22.txt");
    let mut bananas = HashMap::new();
    input.lines().for_each(|line| {
        let mut seen: HashSet<Vec<i32>> = HashSet::new();
        let mut buffer = Vec::new();
        let mut number = line.parse::<u64>().unwrap();
        let mut last_val = None;
        for _ in 0..2000 {
            number = get_next(number);
            let val = (number % 10) as i32;
            if let Some(it) = last_val {
                buffer.push(val - it);
                if buffer.len() == 4 {
                    if !seen.contains(&buffer) {
                        *bananas.entry(buffer.clone()).or_insert(0) += val;
                        seen.insert(buffer.clone());
                    }
                    buffer.remove(0);
                }
            }
            last_val = Some(val);
        }
    });
    result = bananas.values().max().unwrap().clone();

    println!("Result = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        part1();
    }

    #[test]
    fn test_part2() {
        part2();
    }
}
