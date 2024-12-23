use crate::common::read_task;

fn part1() {
    let input = read_task(7);
    let result: u64 = input
        .lines()
        .map(|line| {
            let test_value = line.split_once(":").unwrap().0.parse::<u64>().unwrap();
            let operands = line
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split(" ")
                .map(|operand| operand.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            Task {
                test_value,
                operands,
            }
        })
        .filter(|task| task.part1())
        .map(|task| task.test_value)
        .sum();
    println!("Result: {}", result);
}

struct Task {
    test_value: u64,
    operands: Vec<u64>,
}

impl Task {
    fn part1(&self) -> bool {
        for i in 0..2 << (self.operands.len() - 1) {
            let mut res = self.operands[0];
            for j in 1..self.operands.len() {
                if i & 1 << (j - 1) != 0 {
                    res += self.operands[j];
                } else {
                    res *= self.operands[j];
                }
            }
            if res == self.test_value {
                return true;
            }
        }
        false
    }

    fn part2(&self) -> bool {
        for i in 0..3usize.pow((self.operands.len() - 1) as u32) {
            let mut res = self.operands[0];
            for j in 1..self.operands.len() {
                match (i / 3usize.pow((j - 1) as u32)) % 3 {
                    0 => res += self.operands[j],
                    1 => res *= self.operands[j],
                    2 => res = concatenate(res, self.operands[j]),
                    _ => {
                        panic!("Unexpected value");
                    }
                }
            }
            if res == self.test_value {
                return true;
            }
        }
        false
    }

}

fn concatenate(a: u64, b: u64) -> u64 {
    let x = (b as f64 + 1f64).log10();
    a * 10u64.pow(x.ceil() as u32) + b
}

fn part2() {
    let input = read_task(7);
    let result: u64 = input
        .lines()
        .map(|line| {
            let test_value = line.split_once(":").unwrap().0.parse::<u64>().unwrap();
            let operands = line
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split(" ")
                .map(|operand| operand.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            Task {
                test_value,
                operands,
            }
        })
        .filter(|task| task.part2())
        .map(|task| task.test_value)
        .sum();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(1, 2), 12);
        assert_eq!(concatenate(1, 10), 110);
    }

    #[test]
    fn test_part1() {
        part1();
    }

    #[test]
    fn test_part2() {
        part2();
    }
}
