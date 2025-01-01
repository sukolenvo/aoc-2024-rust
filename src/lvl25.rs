fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl25.txt");

    let schematics = input
        .split("\n\n")
        .map(|group| {
            let data = group
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => 0,
                            '#' => 1,
                            _ => {
                                panic!("unexpected character")
                            }
                        })
                        .collect::<Vec<u16>>()
                })
                .collect::<Vec<Vec<u16>>>();
            Schematic { grid: data }
        })
        .collect::<Vec<Schematic>>();

    let mut lock_counts = vec![vec![vec![vec![vec![0; 6]; 6]; 6]; 6]; 6];

    schematics.iter().filter(|s| s.is_lock()).for_each(|s| {
        lock_counts[s.get_height(0) as usize][s.get_height(1) as usize]
            [s.get_height(2) as usize][s.get_height(3) as usize][s.get_height(4) as usize] += 1;
    });

    schematics.iter().filter(|s| !s.is_lock()).for_each(|s| {
        for pin1 in 0..=5 - s.get_height(0) {
            for pin2 in 0..=5 - s.get_height(1) {
                for pin3 in 0..=5 - s.get_height(2) {
                    for pin4 in 0..=5 - s.get_height(3) {
                        for pin5 in 0..=5 - s.get_height(4) {
                            result += lock_counts[pin1 as usize][pin2 as usize][pin3 as usize]
                                [pin4 as usize][pin5 as usize];
                        }
                    }
                }
            }
        }
    });

    println!("Result = {}", result);
}

struct Schematic {
    grid: Vec<Vec<u16>>,
}

impl Schematic {
    fn is_lock(&self) -> bool {
        self.grid.first().unwrap().iter().all(|&x| x == 1)
    }

    fn get_height(&self, i: usize) -> u16 {
        let mut result = 0;
        for j in 1..self.grid.len() - 1 {
            result += self.grid[j][i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        part1();
    }
}
