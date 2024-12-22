use crate::common::read_task;

pub fn lvl02() {
    let input = read_task(2);
    let mut safe_reports = 0;
    for line in input.lines() {
        if check_levels_safe(line.to_string(), |prev, current| {
            prev + 1 <= current && current <= prev + 3
        }) || check_levels_safe(line.to_string(), |prev, current| {
            prev - 1 >= current && current >= prev - 3
        }) {
            safe_reports += 1;
        }
    }
    println!("Safe reports: {}", safe_reports);
}

fn check_levels_safe(line: String, predicate: fn(i32, i32) -> bool) -> bool {
    let mut levels = line.split_whitespace();
    let mut prev: i32 = levels.next().unwrap().parse().unwrap();
    for level in levels {
        let current: i32 = level.parse().unwrap();
        if !predicate(prev, current) {
            return false;
        }
        prev = current;
    }
    true
}

pub fn lvl02_second() {
    let input = read_task(2);
    let mut safe_reports = 0;
    for line in input.lines() {
        let items: Vec<String> = line.split_whitespace().map(String::from).collect();
        for i in 0..items.len() {
            let subtask: String = items
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, s)| s.clone())
                .collect::<Vec<String>>()
                .join(" ");
            if check_levels_safe(subtask.clone(), |prev, current| {
                prev + 1 <= current && current <= prev + 3
            }) || check_levels_safe(subtask, |prev, current| {
                prev - 1 >= current && current >= prev - 3
            }) {
                safe_reports += 1;
                break;
            }
        }
    }
    println!("Safe reports: {}", safe_reports);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lvl02() {
        lvl02();
    }

    #[test]
    fn test_lvl02_second() {
        lvl02_second();
    }
}