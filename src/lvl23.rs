use std::collections::{HashMap, HashSet};
use std::path::Component::ParentDir;

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl23.txt");
    let mut links = HashMap::new();
    input.lines()
        .for_each(|line| {
            let left_right = line.split("-").collect::<Vec<&str>>();
            links.entry(left_right[0]).or_insert(HashSet::new()).insert(left_right[1]);
            links.entry(left_right[1]).or_insert(HashSet::new()).insert(left_right[0]);
        });
    let mut parties = HashSet::new();
    links.iter()
        .filter(|(key, val)| {
            return key.starts_with("t");
        })
        .for_each(|(key, value)| {
            for neighbor in value {
                for neighbor2 in value {
                    if neighbor2 == neighbor {
                        continue;
                    }
                    if links[neighbor].contains(neighbor2) {
                        let mut party = vec![key, neighbor, neighbor2];
                        party.sort();
                        parties.insert(party);
                    }
                }
            }
        });
    result = parties.len();
    println!("Result = {}", result);
}

fn part2() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl23.txt");
    let mut links = HashMap::new();
    input.lines()
        .for_each(|line| {
            let left_right = line.split("-").collect::<Vec<&str>>();
            links.entry(left_right[0]).or_insert(HashSet::new()).insert(left_right[1]);
            links.entry(left_right[1]).or_insert(HashSet::new()).insert(left_right[0]);
        });
    let mut parties: Vec<Vec<&str>> = Vec::new();
    links.iter()
        .for_each(|(key, value)| {
            for party in &mut parties {
                if party.iter().all(|&member| value.contains(member)) {
                    party.push(key);
                }
            }
            parties.push(vec![*key])
        });
    let result = parties.iter()
        .map(|party| party.len())
        .max()
        .unwrap();
    parties.iter_mut()
        .filter(|party| party.len() == result)
        .for_each(|party| {
            party.sort();
            println!("{:?}", party.join(","));
        });
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
