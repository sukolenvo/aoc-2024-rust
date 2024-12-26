use std::collections::HashMap;

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl19.txt");
    let patterns = input
        .split_once("\n")
        .unwrap()
        .0
        .split(", ")
        .collect::<Vec<&str>>();
    let mut solver = Solver {
        cache: HashMap::new(),
    };
    result = input
        .lines()
        .skip(2)
        .filter(|line| {
            let valid = solver.solve(&patterns, line, 0);
            println!("{} = {}", line, valid);
            return valid;
        })
        .count();
    println!("Result = {}", result);
}

struct Solver {
    cache: HashMap<String, bool>,
}

impl Solver {
    fn solve(&mut self, patterns: &Vec<&str>, line: &str, pos: usize) -> bool {
        if pos == line.len() {
            return true;
        }
        let key = line[pos..].to_string();
        if let Some(&result) = self.cache.get(&key) {
            return result;
        }
        for &pattern in patterns {
            if line[pos..].starts_with(pattern) {
                if self.solve(patterns, line, pos + pattern.len()) {
                    self.cache.insert(key, true);
                    return true;
                }
            }
        }
        self.cache.insert(key, false);
        return false;
    }
}

fn part2() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl19.txt");
    let patterns = input
        .split_once("\n")
        .unwrap()
        .0
        .split(", ")
        .collect::<Vec<&str>>();
    let mut solver = Solver2 {
        cache: HashMap::new(),
    };
    result = input
        .lines()
        .skip(2)
        .map(|line| {
            let valid = solver.solve(&patterns, line, 0);
            println!("{} = {}", line, valid);
            return valid;
        })
        .sum();
    println!("Result = {}", result);
}

struct Solver2 {
    cache: HashMap<String, u64>,
}

impl Solver2 {
    fn solve(&mut self, patterns: &Vec<&str>, line: &str, pos: usize) -> u64 {
        if pos == line.len() {
            return 1;
        }
        let key = line[pos..].to_string();
        if let Some(&result) = self.cache.get(&key) {
            return result;
        }
        let mut result = 0;
        for &pattern in patterns {
            if line[pos..].starts_with(pattern) {
                result += self.solve(patterns, line, pos + pattern.len());
            }
        }
        self.cache.insert(key, result);
        result
    }
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
