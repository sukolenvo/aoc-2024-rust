use std::collections::{HashMap, HashSet};

const SIZE: usize = 71;

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl18.txt");
    let mut obstacles: Vec<Vec<i32>> = vec![vec![0; SIZE]; SIZE];
    let coordinates = input
        .lines()
        .take(1024)
        .map(|line| {
            let xy = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (xy[0], xy[1])
        })
        .collect::<Vec<(usize, usize)>>();
    for &(x, y) in &coordinates {
        obstacles[y][x] += 1;
    }
    let mut positions = HashMap::new();
    positions.insert((0, 0), 0);
    while !positions.contains_key(&(SIZE - 1, SIZE - 1)) {
        for r in 0..obstacles.len() {
            for c in 0..obstacles[r].len() {
                if let Some(&val) = positions.get(&(r, c)) {
                    if r > 0 && obstacles[r - 1][c] == 0 {
                        positions.entry((r - 1, c)).or_insert(val + 1);
                    }
                    if c > 0 && obstacles[r][c - 1] == 0 {
                        positions.entry((r, c - 1)).or_insert(val + 1);
                    }
                    if r < SIZE - 1 && obstacles[r + 1][c] == 0 {
                        positions.entry((r + 1, c)).or_insert(val + 1);
                    }
                    if c < SIZE - 1 && obstacles[r][c + 1] == 0 {
                        positions.entry((r, c + 1)).or_insert(val + 1);
                    }
                }
            }
        }
    }
    result = *positions.get(&(SIZE - 1, SIZE - 1)).unwrap();
    println!("Result = {}", result);
}

fn part2() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl18.txt");
    let mut obstacles: Vec<Vec<i32>> = vec![vec![0; SIZE]; SIZE];
    let coordinates = input
        .lines()
        .map(|line| {
            let xy = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (xy[0], xy[1])
        })
        .collect::<Vec<(usize, usize)>>();
    for &(x, y) in coordinates.iter().take(1024) {
        obstacles[y][x] += 1;
    }
    let mut path = HashSet::new();
    for r in 0..SIZE {
        for c in 0..SIZE {
            if obstacles[r][c] == 0 {
                path.insert((r, c));
            }
        }
    }
    for i in 1024..coordinates.len() {
        println!("{}", i);
        let obstacle = coordinates[i];
        obstacles[obstacle.1][obstacle.0] += 1;
        if path.contains(&(obstacle.1, obstacle.0)) {
            let mut positions = HashMap::new();
            positions.insert((0, 0), vec![(0, 0)]);
            while !positions.contains_key(&(SIZE - 1, SIZE - 1)) {
                let mut found = false;
                for r in 0..obstacles.len() {
                    for c in 0..obstacles[r].len() {
                        let val = positions.get(&(r, c)).unwrap_or(&vec![]).clone();
                        if val.len() > 0 {
                            if r > 0
                                && obstacles[r - 1][c] == 0
                                && !positions.contains_key((&(r - 1, c)))
                            {
                                let mut it = val.clone();
                                it.push((r - 1, c));
                                positions.insert((r - 1, c), it);
                                found = true;
                            }
                            if c > 0
                                && obstacles[r][c - 1] == 0
                                && !positions.contains_key((&(r, c - 1)))
                            {
                                let mut it = val.clone();
                                it.push((r, c - 1));
                                positions.insert((r, c - 1), it);
                                found = true;
                            }
                            if r < SIZE - 1
                                && obstacles[r + 1][c] == 0
                                && !positions.contains_key((&(r + 1, c)))
                            {
                                let mut it = val.clone();
                                it.push((r + 1, c));
                                positions.insert((r + 1, c), it);
                                found = true;
                            }
                            if c < SIZE - 1
                                && obstacles[r][c + 1] == 0
                                && !positions.contains_key((&(r, c + 1)))
                            {
                                let mut it = val.clone();
                                it.push((r, c + 1));
                                positions.insert((r, c + 1), it);
                                found = true;
                            }
                        }
                    }
                }
                if !found {
                    println!("{:?}", obstacle);
                    return;
                }
            }
            path.clear();
            path.extend(positions.get(&(SIZE - 1, SIZE - 1)).unwrap());
        }
    }
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
