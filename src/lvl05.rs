use crate::common::read_task;
use std::collections::{HashMap, HashSet};

pub fn lvl05() {
    let input = read_task(5);
    let dependencies = parse_dependencies(&input);
    let updates = parse_updates(&input);
    let result = updates
        .into_iter()
        .filter(|update| {
            for i in 0..update.len() {
                for j in 0..i {
                    if let Some(dep) = dependencies.get(update.get(j).unwrap()) {
                        if dep.contains(update.get(i).unwrap()) {
                            return false;
                        }
                    }
                }
            }
            return true;
        })
        .map(|update| {
            update
                .get(update.len() / 2)
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>();
    println!("Result: {}", result);
}

fn parse_dependencies(input: &String) -> HashMap<&str, Vec<&str>> {
    let mut direct_dependencies = HashMap::new();
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (left, right) = line.split_once("|").unwrap();
            direct_dependencies.entry(right).or_insert(Vec::new()).push(left);
        });
    direct_dependencies
}

pub fn lvl05_second() {
    let input = read_task(5);
    let dependencies = parse_dependencies(&input);
    let updates = parse_updates(&input);
    let result = updates
        .into_iter()
        .filter(|update| {
            for i in 0..update.len() {
                for j in 0..i {
                    if let Some(dep) = dependencies.get(update.get(j).unwrap()) {
                        if dep.contains(update.get(i).unwrap()) {
                            return true;
                        }
                    }
                }
            }
            return false;
        })
        .map(|update| {
            let mut res = update.clone();
            for mut i in 0..res.len() {
                let test = *res.get(i).unwrap();
                for j in 0..i {
                    if let Some(dep) = dependencies.get(res.get(j).unwrap()) {
                        if dep.contains(&test) {
                            res.remove(i);
                            res.insert(j, test);
                            i = j;
                            break;
                        }
                    }
                }
            }
            return res;
        })
        .map(|update| {
            update
                .get(update.len() / 2)
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>();
    println!("Result: {}", result);
}

fn parse_topological_order(input: &String) -> Vec<&str> {
    let mut vertexes = HashSet::new();
    let mut edges: Vec<(&str, &str)> = Vec::new();
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (left, right) = line.split_once("|").unwrap();
            vertexes.insert(left);
            vertexes.insert(right);
            edges.push((left, right));
        });
    let mut orphan_vertexes = vertexes
        .iter()
        .filter(|&&vertex| edges.iter().all(|&(left, right)| right != vertex))
        .cloned()
        .collect::<Vec<&str>>();
    let mut sorted_vertexes = Vec::new();
    while !orphan_vertexes.is_empty() {
        let vertex = orphan_vertexes.pop().unwrap();
        sorted_vertexes.push(vertex);
        let edges_to_remove = edges
            .iter()
            .filter(|&(left, right)| *left == vertex)
            .cloned()
            .collect::<Vec<(&str, &str)>>();
        edges.retain(|&(left, right)| left != vertex);
        edges_to_remove.into_iter().for_each(|(_, check_vertex)| {
            if edges.iter().all(|&(_, right)| right != check_vertex) {
                orphan_vertexes.push(check_vertex);
            }
        });
    }
    assert!(edges.is_empty(), "all edges should be removed");
    assert_eq!(sorted_vertexes.len(), vertexes.len(), "all vertexes should be sorted");
    sorted_vertexes
}

fn parse_updates(input: &String) -> Vec<Vec<&str>> {
    input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lvl05() {
        lvl05();
    }

    #[test]
    fn test_lvl05_second() {
        lvl05_second();
    }
}