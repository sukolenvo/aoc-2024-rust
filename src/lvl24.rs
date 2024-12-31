use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use rand::Rng;

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl24.txt");
    let values_relations = input.split("\n\n").collect::<Vec<&str>>();
    let mut operand_to_result = HashMap::new();
    let mut operations: HashMap<&str, (&str, &str, fn(u16, u16) -> u16)> = HashMap::new();
    values_relations[1].lines().for_each(|line| {
        let val = line.split(" ").collect::<Vec<&str>>();
        let op1 = val[0];
        let op2 = val[2];
        let op3 = val[4];
        let operation = val[1];
        operand_to_result.entry(op1).or_insert(Vec::new()).push(op3);
        operand_to_result.entry(op2).or_insert(Vec::new()).push(op3);
        match operation {
            "XOR" => {
                operations.insert(op3, (op1, op2, |a: u16, b: u16| a ^ b));
            }
            "AND" => {
                operations.insert(op3, (op1, op2, |a: u16, b: u16| a & b));
            }
            "OR" => {
                operations.insert(op3, (op1, op2, |a: u16, b: u16| a | b));
            }
            &_ => {
                panic!("Unknown operation");
            }
        }
    });
    let mut known_values: HashMap<String, u16> = HashMap::new();
    values_relations[0].lines().for_each(|line| {
        let wire_value = line.split(": ").collect::<Vec<&str>>();
        let wire = wire_value[0];
        let value = wire_value[1].parse::<u16>().unwrap();
        insert_value(
            &mut known_values,
            wire,
            value,
            &operand_to_result,
            &operations,
        );
    });
    let mut bits = vec![];
    for i in 0.. {
        let op = format!("z{:02}", i);
        if operations.contains_key(op.as_str()) {
            bits.push(known_values.get(op.as_str()).unwrap());
        } else {
            break;
        }
    }
    bits.reverse();
    println!("Bits = {:?}", bits);
    println!("Result = {}", result);
}

fn insert_value<'a>(
    known_values: &mut HashMap<String, u16>,
    wire: &'a str,
    value: u16,
    operand_to_result: &HashMap<&str, Vec<&'a str>>,
    operations: &HashMap<&str, (&str, &str, fn(u16, u16) -> u16)>,
) {
    println!("Inserting {} = {}", wire, value);
    known_values.insert(wire.to_string(), value);
    if let Some(dependent_wires) = operand_to_result.get(wire) {
        dependent_wires.iter().for_each(|dependent_wire| {
            let &(op1, op2, operation) = operations.get(dependent_wire).unwrap();
            if op1 == wire && known_values.contains_key(op2) {
                let op2_value = known_values.get(op2).unwrap();
                let result = operation(value, *op2_value);
                insert_value(
                    known_values,
                    dependent_wire,
                    result,
                    operand_to_result,
                    operations,
                );
            } else if op2 == wire && known_values.contains_key(op1) {
                let op1_value = known_values.get(op1).unwrap();
                let result = operation(*op1_value, value);
                insert_value(
                    known_values,
                    dependent_wire,
                    result,
                    operand_to_result,
                    operations,
                );
            }
        });
    }
}

fn part2() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl24.txt");
    let values_relations = input.split("\n\n").collect::<Vec<&str>>();
    let mut operand_to_result = HashMap::new();
    let mut operations: HashMap<String, (&str, &str, fn(u16, u16) -> u16)> = HashMap::new();
    values_relations[1].lines().for_each(|line| {
        let val = line.split(" ").collect::<Vec<&str>>();
        let op1 = val[0];
        let op2 = val[2];
        let op3 = val[4];
        let operation = val[1];
        operand_to_result
            .entry(op1.to_string())
            .or_insert(Vec::new())
            .push(op3.to_string());
        operand_to_result
            .entry(op2.to_string())
            .or_insert(Vec::new())
            .push(op3.to_string());
        match operation {
            "XOR" => {
                operations.insert(op3.to_string(), (op1, op2, |a: u16, b: u16| a ^ b));
            }
            "AND" => {
                operations.insert(op3.to_string(), (op1, op2, |a: u16, b: u16| a & b));
            }
            "OR" => {
                operations.insert(op3.to_string(), (op1, op2, |a: u16, b: u16| a | b));
            }
            &_ => {
                panic!("Unknown operation");
            }
        }
    });

    let size = 45;
    let outputs = values_relations[1]
        .lines()
        .map(|line| {
            let val = line.split(" ").collect::<Vec<&str>>();
            val[4].to_string()
        })
        .collect::<Vec<String>>();

    let mut valid = HashSet::new();

    for i in 0..size {
        valid.insert(format!("x{:02}", i).to_string());
        valid.insert(format!("y{:02}", i).to_string());
    }

    let candidate: Vec<(&str, &str)> = vec![
    ];
    let mut swaps = vec![];
    for (l, r) in candidate {
        swaps.push((l.to_string(), r.to_string()));
        let op1 = *operations.get(l).unwrap();
        let op2 = *operations.get(r).unwrap();
        operations.insert(l.to_string(), op2);
        operations.insert(r.to_string(), op1);
        valid.insert(l.to_string());
        valid.insert(r.to_string());
    }
    let ok = process_2(0, size, &mut operations, &mut valid, &outputs, &mut swaps);
    assert!(ok, "no solution found");
    println!("Swaps = {:?}", swaps);
    let mut wires = swaps
        .into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<String>>();
    wires.sort();
    println!("Wires = {:?}", wires.join(","));
    println!("Result = {}", result);
}

fn process_2(
    i: usize,
    size: usize,
    operations: &mut HashMap<String, (&str, &str, fn(u16, u16) -> u16)>,
    valid: &mut HashSet<String>,
    outputs: &Vec<String>,
    swaps: &mut Vec<(String, String)>,
) -> bool {
    if i == size {
        let mut wires = swaps
            .iter()
            .flat_map(|(a, b)| vec![a, b])
            .cloned()
            .collect::<Vec<String>>();
        wires.sort();
        println!("Verifying solution {:?}", wires.join(","));
        for val in vec![0, 1] {
            let mut known_values: HashMap<String, u16> = HashMap::new();
            for j in 0..size {
                let key = format!("x{:02}", j);
                known_values.insert(key, val);
                let key = format!("y{:02}", j);
                known_values.insert(key, val);
            }

            solve_2(&mut known_values, operations);
            for j in 1..=i {
                if known_values.get(&format!("z{:02}", j)) != Some(&val) {
                    return false;
                }
            }
        }
        let mut rng = rand::thread_rng();
        for j in 0..2usize.pow(size as u32) as usize {
            let random_number: u64 = rng.gen_range(0..(1 << size));
            let random_number2: u64 = rng.gen_range(0..(1 << size));
            let mut known_values: HashMap<String, u16> = HashMap::new();
            for k in 0..size {
                let key = format!("x{:02}", k);
                known_values.insert(key, ((random_number >> k) & 1) as u16);
                let key = format!("y{:02}", k);
                known_values.insert(key, ((random_number2 >> k) & 1) as u16);
            }
            solve_2(&mut known_values, operations);
            for k in 0..=i {
                if known_values.get(&format!("z{:02}", k)) != Some(&((((random_number+random_number2) >> k) & 1) as u16)) {
                    println!("{}+{} failed", random_number, random_number2);
                    return false;
                }
            }
            if j % 1000 == 0 {
                println!("{} ok", j);
            }
        }
        return true;
    }
    let mut wires_to_test = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(format!("z{:02}", i));
    while !queue.is_empty() {
        let wire = queue.pop_front().unwrap();
        if wires_to_test.contains(&wire) {
            continue;
        }
        wires_to_test.insert(wire.clone());
        if let Some(results) = operations.get(&wire) {
            queue.push_back(results.0.to_string());
            queue.push_back(results.1.to_string());
        }
    }
    wires_to_test.retain(|wire| !valid.contains(wire));
    let val = check_i(i, &operations);
    if val {
        valid.extend(wires_to_test.clone());
        println!("{}: {:?}", i, valid.len());
        if process_2(i + 1, size, operations, valid, outputs, swaps) {
            return true;
        }
        valid.retain(|state| !wires_to_test.contains(state));
        false
    } else {
        if swaps.len() == 4 {
            return false;
        }
        let mut iter = 0;
        let iter_count = wires_to_test.len() * outputs.len();
        for first in &wires_to_test {
            if valid.contains(first) {
                iter += outputs.len();
                continue;
            }
            for second in outputs {
                if iter % 100 == 0 {
                    println!("{}: {}/{}", i, iter, iter_count);
                }
                iter += 1;
                if valid.contains(second) {
                    continue;
                }
                let inputs_first = *operations.get(first.as_str()).unwrap();
                let inputs_second = *operations.get(second.as_str()).unwrap();
                operations.insert(first.clone(), inputs_second);
                operations.insert(second.clone(), inputs_first);
                valid.insert(first.clone());
                valid.insert(second.clone());
                if !check_i(i, &operations) {
                    valid.remove(first);
                    valid.remove(second);
                    operations.insert(first.clone(), inputs_first);
                    operations.insert(second.clone(), inputs_second);
                    continue;
                }

                swaps.push((first.clone(), second.clone()));
                if process_2(i + 1, size, operations, valid, outputs, swaps) {
                    return true;
                }
                swaps.pop();
                valid.remove(first);
                valid.remove(second);
                operations.insert(first.clone(), inputs_first);
                operations.insert(second.clone(), inputs_second);
            }
        }
        false
    }
}

fn check_i(i: usize, gates: &HashMap<String, (&str, &str, fn(u16, u16) -> u16)>) -> bool {
    let tests = vec![(1, 1, 0, 1), (1, 0, 1, 0), (0, 1, 1, 0), (0, 0, 0, 0)];
    for test in &tests {
        let x_key = format!("x{:02}", i);
        let y_key = format!("y{:02}", i);
        let mut known_values: HashMap<String, u16> = HashMap::new();
        for j in 0..i + 2 {
            let key = format!("x{:02}", j);
            known_values.insert(key, 0);
            let key = format!("y{:02}", j);
            known_values.insert(key, 0);
        }
        known_values.insert(x_key.clone(), test.0);
        known_values.insert(y_key.clone(), test.1);

        solve_2(&mut known_values, gates);
        for j in 0..=i {
            let val = known_values.get(&format!("z{:02}", j));
            if j < i && val != Some(&0) {
                return false;
            } else if j == i && val != Some(&test.2) {
                return false;
            } else if j == i + 1 && val != Some(&test.3) {
                return false;
            }
        }
    }
    true
}

fn solve_2(
    state: &mut HashMap<String, u16>,
    gates: &HashMap<String, (&str, &str, fn(u16, u16) -> u16)>,
) {
    loop {
        let mut found = 0;
        for (op3, (op1, op2, operation)) in gates.iter() {
            if state.contains_key(op3.as_str()) {
                continue;
            }
            let op1_value = match state.get(&op1.to_string()) {
                Some(value) => *value,
                None => continue,
            };
            let op2_value = match state.get(&op2.to_string()) {
                Some(value) => *value,
                None => continue,
            };
            let result = operation(op1_value, op2_value);
            state.insert(op3.to_string(), result);
            found += 1;
        }
        if found == 0 {
            break;
        }
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
