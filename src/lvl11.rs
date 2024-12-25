use std::collections::HashMap;
use crate::common::read_task;

fn part1() {
    let input = read_task(11);
    let stones = input.split(" ").map(|x| Stone { val: x.parse().unwrap() }).collect::<Vec<Stone>>();
    let mut result = 0;
    let mut evo = Evo { cache: HashMap::new() };
    for stone in stones {
        result += evo.count_stones(&stone, 25);
    }
    println!("Result: {}", result);
}

struct Evo {
    cache: HashMap<(u64, u32), u64>,
}

impl Evo {

    fn count_stones(&mut self, stone: &Stone, iterations: u32) -> u64 {
        if iterations == 0 {
            return 1;
        }
        if let Some(&cached) = self.cache.get(&(stone.val, iterations)) {
            return cached;
        }
        let result = stone.mutate().iter().map(|x| self.count_stones(x, iterations - 1)).sum();
        self.cache.insert((stone.val, iterations), result);
        result
    }
}

struct Stone {
    val: u64,
}

impl Stone {
    fn mutate(&self) -> Vec<Stone> {
        if self.val == 0 {
            vec![Stone { val: 1 }]
        } else if count_digits(self.val) % 2 == 0 {
            let digit_count = count_digits(self.val);
            return vec![Stone { val: self.val / 10u64.pow(digit_count / 2) }, Stone { val: self.val % 10u64.pow(digit_count / 2) }];
        } else {
            return vec![Stone { val: self.val * 2024 }];
        }
    }
}

fn count_digits(num: u64) -> u32 {
    (num as f64 + 1f64).log10().ceil() as u32
}

fn part2() {
    let input = read_task(11);
    let stones = input.split(" ").map(|x| Stone { val: x.parse().unwrap() }).collect::<Vec<Stone>>();
    let mut result = 0;
    let mut evo = Evo { cache: HashMap::new() };
    for stone in stones {
        result += evo.count_stones(&stone, 75);
    }
    println!("Result: {}", result);
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
