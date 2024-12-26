fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl17.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let a = lines[0]
        .split(": ")
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .next()
        .unwrap();
    let b = lines[1]
        .split(": ")
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .next()
        .unwrap();
    let c = lines[2]
        .split(": ")
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .next()
        .unwrap();
    let instructions = lines[4]
        .split(": ")
        .skip(1)
        .flat_map(|x| x.split(","))
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    run_program(a, b, c, 0, &instructions);
    println!("\nResult = {}", result);
}

const ADV: i32 = 0;
const BXL: i32 = 1;
const BST: i32 = 2;
const JNZ: i32 = 3;
const BXC: i32 = 4;
const OUT: i32 = 5;
const BDV: i32 = 6;
const CDV: i32 = 7;

fn run_program(a: i64, b: i64, c: i64, ip: usize, instructions: &Vec<i32>) {
    if ip >= instructions.len() {
        return;
    }
    let instruction = instructions[ip];
    let operand = instructions[ip + 1];
    match instruction {
        ADV => {
            let val = get_combo_value(a, b, c, operand);
            let divisor = 2i64.pow(val as u32);
            run_program(a / divisor, b, c, ip + 2, instructions);
        }
        BXL => {
            run_program(a, b ^ operand as i64, c, ip + 2, instructions);
        }
        BST => {
            let val = get_combo_value(a, b, c, operand);
            run_program(a, val % 8, c, ip + 2, instructions);
        }
        JNZ => {
            if a == 0 {
                run_program(a, b, c, ip + 2, instructions);
            } else {
                run_program(a, b, c, operand as usize, instructions);
            }
        }
        BXC => {
            run_program(a, b ^ c, c, ip + 2, instructions);
        }
        OUT => {
            let val = get_combo_value(a, b, c, operand);
            print!("{},", val % 8);
            run_program(a, b, c, ip + 2, instructions);
        }
        BDV => {
            let val = get_combo_value(a, b, c, operand);
            let divisor = 2i64.pow(val as u32);
            run_program(a, a / divisor, c, ip + 2, instructions);
        }
        CDV => {
            let val = get_combo_value(a, b, c, operand);
            let divisor = 2i64.pow(val as u32);
            run_program(a, b, a / divisor, ip + 2, instructions);
        }
        _ => {
            panic!("Unknown instruction");
        }
    }
}

fn get_combo_value(a: i64, b: i64, c: i64, operand: i32) -> i64 {
    if operand <= 3 {
        return operand as i64;
    }
    match operand {
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Unknown combo operand"),
    }
}

fn part2_naive() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl17.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let a = lines[0]
        .split(": ")
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .next()
        .unwrap();
    let b = lines[1]
        .split(": ")
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .next()
        .unwrap();
    let c = lines[2]
        .split(": ")
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .next()
        .unwrap();
    let instructions = lines[4]
        .split(": ")
        .skip(1)
        .flat_map(|x| x.split(","))
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for i in 35184372088832.. {
        match run_program_2(i, b, c, 0, &instructions, 0, 0) {
            Ok(_) => {
                result = i;
                break;
            }
            Err(_) => {
                if i % 1000 == 0 {
                    println!("i = {}", i);
                }
            }
        }
    }
    println!("\nResult = {}", result);
}

fn run_program_2(
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    instructions: &Vec<i32>,
    output_idx: usize,
    steps: usize,
) -> Result<(), ()> {
    if steps > 100 && output_idx == 0 {
        return Err(());
    }
    if ip >= instructions.len() {
        return Err(());
    }
    let instruction = instructions[ip];
    let operand = instructions[ip + 1];
    match instruction {
        ADV => {
            let val = get_combo_value(a, b, c, operand);
            let divisor = 2i64.pow(val as u32);
            run_program_2(
                a / divisor,
                b,
                c,
                ip + 2,
                instructions,
                output_idx,
                steps + 1,
            )
        }
        BXL => run_program_2(
            a,
            b ^ operand as i64,
            c,
            ip + 2,
            instructions,
            output_idx,
            steps + 1,
        ),
        BST => {
            let val = get_combo_value(a, b, c, operand);
            run_program_2(a, val % 8, c, ip + 2, instructions, output_idx, steps + 1)
        }
        JNZ => {
            if a == 0 {
                run_program_2(a, b, c, ip + 2, instructions, output_idx, steps + 1)
            } else {
                run_program_2(
                    a,
                    b,
                    c,
                    operand as usize,
                    instructions,
                    output_idx,
                    steps + 1,
                )
            }
        }
        BXC => run_program_2(a, b ^ c, c, ip + 2, instructions, output_idx, steps + 1),
        OUT => {
            let val = get_combo_value(a, b, c, operand) % 8;
            if (val as i32 != instructions[output_idx]) {
                return Err(());
            }
            if output_idx == instructions.len() - 1 {
                println!("\nFound");
                return Ok(());
            }
            run_program_2(a, b, c, ip + 2, instructions, output_idx + 1, steps + 1)
        }
        BDV => {
            let val = get_combo_value(a, b, c, operand);
            let divisor = 2i64.pow(val as u32);
            run_program_2(
                a,
                a / divisor,
                c,
                ip + 2,
                instructions,
                output_idx,
                steps + 1,
            )
        }
        CDV => {
            let val = get_combo_value(a, b, c, operand);
            let divisor = 2i64.pow(val as u32);
            run_program_2(
                a,
                b,
                a / divisor,
                ip + 2,
                instructions,
                output_idx,
                steps + 1,
            )
        }
        _ => {
            panic!("Unknown instruction");
        }
    }
}

fn part2() {
    let target = vec![2,4,1,5,7,5,0,3,1,6,4,3,5,5,3,0];
    let mut last_i = 0;
    let mut i = 35297261291930;
    let mut samples = 1;
    let mut completed = 0;
    loop {
        let mut a = i;
        let mut idx = 0;
        while a > 0 {
            let mut b = a % 8;
            b = b ^ 5;
            let c = a / 2i64.pow(b as u32);
            b = b ^ 6;
            b = b ^ c;
            b = b % 8;
            if b != target[idx] {
                break;
            }
            idx += 1;
            if idx == target.len() {
                println!("diff {}", i - last_i);
                last_i = i;
                samples -= 1;
                if samples == 0 {
                    return;
                }
                break;
            }
            a = a / 8;
        }
        i += 65536;
        completed+= 1;
        if completed % 1_000_000 == 0 {
            println!("i = {}", i);
        }
    }
}

// BST 4  b = a % 8
// BXL 5  b = b ^ 5
// CDV 5  c = a / 2**b
// ADV 3  a = a / 2**3
// BXL 6  b = b ^ 6
// BXC 3  b = b ^ c
// OUT 5 print b % 8
// JNZ 0


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
