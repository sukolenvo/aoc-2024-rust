use crate::common::read_task;

fn part1() {
    let input = read_task(10);
    let map = Map::new(&input);
    let mut result = 0;
    for r in 0..map.data.len() {
        for c in 0..map.data[0].len() {
            if map.data[r][c] == 0 {
                result += map.walk(r, c);
            }
        }
    }
    println!("Result: {}", result);
}

struct Map {
    data: Vec<Vec<u32>>,
}

impl Map {
    fn new(input: &String) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        Self { data }
    }

    fn walk(&self, r: usize, c: usize) -> u32 {
        assert_eq!(self.data[r][c], 0);
        let mut positions = vec![(0, r, c)];
        let mut process = 0;
        loop {
            if (process == positions.len()) {
                break;
            }
            let (height, r, c) = positions[process];
            let neighbors = vec![
                (r as i32 - 1, c as i32),
                (r as i32 + 1, c as i32),
                (r as i32, c as i32 - 1),
                (r as i32, c as i32 + 1),
            ];
            for (nr, nc) in neighbors {
                if (nr < 0) || (nr >= self.data.len() as i32) || (nc < 0) || (nc >= self.data[0].len() as i32) {
                    continue;
                }
                if self.data[nr as usize][nc as usize] == height + 1 {
                    if !positions.contains(&(height + 1, nr as usize, nc as usize)) {
                        positions.push((height + 1, nr as usize, nc as usize));
                    }
                }
            }
            process += 1;
        }
        positions.iter().filter(|(h, _, _)| *h == 9).count() as u32
    }

    fn walk2(&self, r: usize, c: usize) -> u32 {
        assert_eq!(self.data[r][c], 0);
        let mut positions = vec![(0, r, c)];
        let mut process = 0;
        loop {
            if (process == positions.len()) {
                break;
            }
            let (height, r, c) = positions[process];
            let neighbors = vec![
                (r as i32 - 1, c as i32),
                (r as i32 + 1, c as i32),
                (r as i32, c as i32 - 1),
                (r as i32, c as i32 + 1),
            ];
            for (nr, nc) in neighbors {
                if (nr < 0) || (nr >= self.data.len() as i32) || (nc < 0) || (nc >= self.data[0].len() as i32) {
                    continue;
                }
                if self.data[nr as usize][nc as usize] == height + 1 {
                    positions.push((height + 1, nr as usize, nc as usize));
                }
            }
            process += 1;
        }
        positions.iter().filter(|(h, _, _)| *h == 9).count() as u32
    }
}

fn part2() {
    let input = read_task(10);
    let map = Map::new(&input);
    let mut result = 0;
    for r in 0..map.data.len() {
        for c in 0..map.data[0].len() {
            if map.data[r][c] == 0 {
                result += map.walk2(r, c);
            }
        }
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