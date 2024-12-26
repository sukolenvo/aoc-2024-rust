use std::collections::HashMap;

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl21.txt");
    result = input
        .lines()
        .map(|line| {
            let keypad = NumericKeypad {};
            let code = keypad.enter_code(line);
            println!("{}",  code);
            let mut directional_keypad = DirectionalKeypad::new();
            let code2 = directional_keypad.enter_code(code.as_str());
            let code3 = directional_keypad.enter_code(code2.as_str());
            let code4 = directional_keypad.enter_code(code3.as_str());
            let code5 = directional_keypad.enter_code(code4.as_str());
            let code6 = directional_keypad.enter_code(code5.as_str());
            let code7 = directional_keypad.enter_code(code6.as_str());
            let code8 = directional_keypad.enter_code(code7.as_str());
            println!("{} -> {}", line, code3);
            return line[0..3].parse::<usize>().unwrap() * code8.len();
        })
        .sum();
    println!("Result = {}", result);
}

// 379A   3            9                 7
//        ^   A       ^^        <<       A>>AvvvA
//    <   A > A   <   AA  v <   AA >>  ^ AvAA^Av<AAA>^A
// v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A
// <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//    <   A > A  v <<   AA >  ^ AA > A
//        ^   A         <<      ^^   A
//            3                      7
struct NumericKeypad {}

impl NumericKeypad {
    fn enter_code(&self, code: &str) -> String {
        let mut result = String::new();
        let mut pos = self.get_pos('A');
        for c in code.chars() {
            let new_pos = self.get_pos(c);
            if pos.0 != 3 || new_pos.1 != 0 {
                while new_pos.1 < pos.1 {
                    result.push('<');
                    pos.1 -= 1;
                }
            }

            while new_pos.0 < pos.0 {
                result.push('^');
                pos.0 -= 1;
            }
            while new_pos.1 > pos.1 {
                result.push('>');
                pos.1 += 1;
            }
            while new_pos.0 > pos.0 {
                result.push('v');
                pos.0 += 1;
            }
            while new_pos.1 < pos.1 {
                result.push('<');
                pos.1 -= 1;
            }
            result.push('A');
        }
        result
    }

    fn get_pos(&self, c: char) -> (usize, usize) {
        match c {
            '0' => (3, 1),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            'A' => (3, 2),
            _ => panic!("Invalid character"),
        }
    }
}

struct DirectionalKeypad {
    cache: HashMap<String, String>,
}

impl DirectionalKeypad {
    fn new() -> Self {
        DirectionalKeypad {
            cache: HashMap::new(),
        }
    }

    fn enter_code(&mut self, code: &str) -> String {
        let mut result = String::new();
        let idx = code
            .chars()
            .enumerate()
            .filter(|(i, c)| *c == 'A')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        for i in 0..idx.len() {
            let start = if i == 0 { 0 } else { idx[i - 1] + 1 };
            let end = idx[i];
            let chunk = self.enter_code_chunk(&code[start..=end], "v><^");
            result.push_str(chunk.as_str());
        }
        result
    }

    fn enter_code_chunk(&mut self, code: &str, strategy: &str) -> String {
        if let Some(result) = self.cache.get(code) {
            return result.clone();
        }
        let mut result = String::new();
        let mut pos = self.get_pos('A');
        for c in code.chars() { // 9738232
            let new_pos = self.get_pos(c);
            if pos == (1, 1) && new_pos == (0, 2) {
                result.push('^');
                pos.0 -= 1;
                result.push('>');
                pos.1 += 1;
            }
            if pos == (0, 1) && new_pos == (1, 2) {
                result.push('v');
                pos.0 += 1;
                result.push('>');
                pos.1 += 1;
            }

            if pos == (1, 2) && new_pos == (0, 1) {
                result.push('<');
                pos.1 -= 1;
                result.push('^');
                pos.0 -= 1;
            }

            if pos ==(0, 2) && new_pos == (1, 1) {
                result.push('<');
                pos.1 -= 1;
                result.push('v');
                pos.0 += 1;

            }
            for s in strategy.chars() {
                match s {
                    'v' => {
                        while new_pos.0 > pos.0 {
                            result.push('v');
                            pos.0 += 1;
                        }
                    }
                    '>' => {
                        while new_pos.1 > pos.1 {
                            result.push('>');
                            pos.1 += 1;
                        }
                    }
                    '<' => {
                        while new_pos.1 < pos.1 {
                            result.push('<');
                            pos.1 -= 1;
                        }
                    }
                    '^' => {
                        while new_pos.0 < pos.0 {
                            result.push('^');
                            pos.0 -= 1;
                        }
                    }
                    _ => {
                        panic!("unknown strategy item")
                    }
                }
            }
            result.push('A');
        }
        self.cache.insert(code.to_string(), result.clone());
        result
    }

    fn get_pos(&self, c: char) -> (usize, usize) {
        match c {
            '^' => (0, 1),
            '<' => (1, 0),
            '>' => (1, 2),
            'v' => (1, 1),
            'A' => (0, 2),
            _ => panic!("Invalid character"),
        }
    }
}

fn part2() {
    let mut result = 0usize;
    let input = include_str!("../tasks/lvl21.txt");
    result = input
        .lines()
        .map(|line| {
            let keypad = NumericKeypad {};
            let mut codes = HashMap::new();
            for code in split_code(&keypad.enter_code(line)) {
                *codes.entry(code.to_string()).or_insert(0) += 1;
            }
            let mut directional_keypad = DirectionalKeypad::new();
            for i in 0..25 {
                let mut new_codes = HashMap::new();
                codes.iter().for_each(|(code, count)| {
                    let new_code = directional_keypad.enter_code(code.as_str());
                    split_code(&new_code).iter().for_each(|&new_code| {
                        *new_codes.entry(new_code.to_string()).or_insert(0) += count;
                    });
                });
                codes = new_codes;
                // println!("{} -> {:?}", i, codes.keys());
            }
            let code_mul: usize = codes
                .iter()
                .map(|(code, &count)| {
                    return code.len() * count;
                })
                .sum();
            return line[0..3].parse::<usize>().unwrap() * code_mul;
        })
        .sum();
    println!("Result = {}", result);
}

fn split_code(str: &String) -> Vec<&str> {
    let idx = str
        .chars()
        .enumerate()
        .filter(|(i, c)| *c == 'A')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    return idx
        .iter()
        .enumerate()
        .map(|(i, pos)| {
            let start = if i == 0 { 0 } else { idx[i - 1] + 1 };
            let end = *pos;
            &str[start..=end]
        })
        .collect();
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
