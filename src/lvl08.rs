use crate::common::read_task;
use std::collections::{HashMap, HashSet};

fn part1() {
    let input = read_task(8);
    let map = Map::new(&input);
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut antinodes = HashSet::new();
    for positions in map.data.values() {
        for first in positions {
            for second in positions {
                if first != second {
                    let dx = first.0 as isize - second.0 as isize;
                    let dy = first.1 as isize - second.1 as isize;
                    let x1 = first.0 as isize + dx;
                    let y1 = first.1 as isize + dy;
                    if x1 >= 0 && x1 < width as isize && y1 >= 0 && y1 < height as isize {
                        antinodes.insert((x1 as usize, y1 as usize));
                    }
                    let x2 = second.0 as isize - dx;
                    let y2 = second.1 as isize - dy;
                    if x2 >= 0 && x2 < width as isize && y2 >= 0 && y2 < height as isize {
                        antinodes.insert((x2 as usize, y2 as usize));
                    }
                }
            }
        }
    }
    println!("Result: {}", antinodes.len());
}

struct Map {
    data: HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
    fn new(input: &String) -> Self {
        let alphabet = input
            .chars()
            .filter(|c| *c != '.')
            .collect::<HashSet<char>>();
        let lines = input.lines().collect::<Vec<&str>>();
        Self {
            data: alphabet
                .iter()
                .map(|c| {
                    let mut result = Vec::new();
                    for (row, line) in lines.iter().enumerate() {
                        for (column, ch) in line.chars().enumerate() {
                            if ch == *c {
                                result.push((row, column));
                            }
                        }
                    }
                    (*c, result)
                })
                .collect::<HashMap<char, Vec<(usize, usize)>>>(),
        }
    }
}

fn part2() {
    let input = read_task(8);
    let map = Map::new(&input);
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut antinodes = HashSet::new();
    for positions in map.data.values() {
        for first in positions {
            for second in positions {
                if first != second {
                    let dx = first.0 as isize - second.0 as isize;
                    let dy = first.1 as isize - second.1 as isize;
                    let mut i = (first.0 as isize, first.1 as isize);
                    antinodes.insert(*first);
                    loop {
                        i = (i.0 + dx, i.1 + dy);
                        if i.0 < 0 || i.0 >= width as isize || i.1 < 0 || i.1 >= height as isize {
                            break;
                        }
                        antinodes.insert((i.0 as usize, i.1 as usize));
                    }
                    loop {
                        i = (i.0 - dx, i.1 - dy);
                        if i.0 < 0 || i.0 >= width as isize || i.1 < 0 || i.1 >= height as isize {
                            break;
                        }
                        antinodes.insert((i.0 as usize, i.1 as usize));
                    }
                }
            }
        }
    }
    println!("Result: {}", antinodes.len());
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
