use std::collections::HashSet;

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl15.txt");
    let map_instructions = input.split("\n\n").collect::<Vec<&str>>();
    let mut robot = map_instructions[0]
        .lines()
        .enumerate()
        .flat_map(|(r, line)| line.find('@').map(|c| (r, c)))
        .next()
        .unwrap();
    let mut boxes = map_instructions[0]
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, char)| char == 'O')
                .map(|(c, _)| (r, c))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();
    let walls = map_instructions[0]
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, char)| char == '#')
                .map(|(c, _)| (r, c))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    for instruction in map_instructions[1].chars() {
        match instruction {
            '^' => {
                for i in 1.. {
                    if robot.0 < i {
                        panic!("Robot is out of bounds");
                    }
                    let next = (robot.0 - i, robot.1);
                    if walls.contains(&next) {
                        break;
                    }
                    if !boxes.contains(&next) {
                        for j in 1..i {
                            boxes.iter_mut().find(|it| **it == (robot.0 - j, robot.1)).unwrap().0 -= 1;
                        }
                        robot.0 -= 1;
                        break;
                    }
                }
            }
            'v' => {
                for i in 1.. {
                    if robot.0 + i >= map_instructions[0].lines().count() {
                        panic!("Robot is out of bounds");
                    }
                    let next = (robot.0 + i, robot.1);
                    if walls.contains(&next) {
                        break;
                    }
                    if !boxes.contains(&next) {
                        for j in 1..i {
                            boxes.iter_mut().find(|it| **it == (robot.0 + j, robot.1)).unwrap().0 += 1;
                        }
                        robot.0 += 1;
                        break;
                    }
                }
            }
            '<' => {
                for i in 1.. {
                    if robot.1 < i {
                        panic!("Robot is out of bounds");
                    }
                    let next = (robot.0, robot.1 - i);
                    if walls.contains(&next) {
                        break;
                    }
                    if !boxes.contains(&next) {
                        for j in 1..i {
                            boxes.iter_mut().find(|it| **it == (robot.0, robot.1 - j)).unwrap().1 -= 1;
                        }
                        robot.1 -= 1;
                        break;
                    }
                }
            }
            '>' => {
                for i in 1.. {
                    if robot.1 + i >= map_instructions[0].lines().next().unwrap().chars().count() {
                        panic!("Robot is out of bounds");
                    }
                    let next = (robot.0, robot.1 + i);
                    if walls.contains(&next) {
                        break;
                    }
                    if !boxes.contains(&next) {
                        for j in 1..i {
                            boxes.iter_mut().find(|it| **it == (robot.0, robot.1 + j)).unwrap().1 += 1;
                        }
                        robot.1 += 1;
                        break;
                    }
                }
            }
            '\n' => {}
            _ => {
                println!("Unknown instruction: {}", instruction);
            }
        }
    }
    result = boxes.iter().map(|boks| boks.0 * 100 + boks.1).sum();
    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct WideBox {
    row: usize,
    col: usize,
}

impl WideBox {
    fn is_occupying(&self, row: usize, col: usize) -> bool {
        self.row == row && self.col == col || self.row == row && self.col + 1 == col
    }
}

fn part2() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl15.txt");
    let map_instructions = input.split("\n\n").collect::<Vec<&str>>();
    let map_rows = map_instructions[0].lines().count();
    let map_cols = map_instructions[0].lines().next().unwrap().chars().count() * 2;
    let mut robot = map_instructions[0]
        .lines()
        .enumerate()
        .flat_map(|(r, line)| line.find('@').map(|c| (r, c * 2)))
        .next()
        .unwrap();
    let mut boxes = map_instructions[0]
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, char)| char == 'O')
                .map(|(c, _)| WideBox { row: r, col: c * 2 })
                .collect::<Vec<WideBox>>()
        })
        .collect::<Vec<WideBox>>();
    let walls = map_instructions[0]
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, char)| char == '#')
                .flat_map(|(c, _)| vec![(r, 2 * c), (r, 2 * c + 1)])
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    for instruction in map_instructions[1].chars() {
        match instruction {
            '^' => {
                if !walls.contains(&(robot.0 - 1, robot.1)) {
                    let mut boxes_to_move = boxes.iter()
                        .enumerate()
                        .filter(|(_, b)| b.is_occupying(robot.0 - 1, robot.1))
                        .map(|(i, _)| i)
                        .collect::<HashSet<usize>>();
                    loop {
                        let check_coordinates = boxes_to_move.iter()
                            .map(|&i| &boxes[i])
                            .flat_map(|b| vec![(b.row - 1, b.col), (b.row - 1, b.col + 1)])
                            .collect::<HashSet<(usize, usize)>>();
                        if walls.iter().any(|&w| check_coordinates.contains(&w)) {
                            break;
                        }
                        let new_boxes = boxes.iter()
                            .enumerate()
                            .filter(|(_, b)| check_coordinates.iter().any(|&c| b.is_occupying(c.0, c.1)))
                            .map(|(i, _)| i)
                            .filter(|&i| !boxes_to_move.contains(&i))
                            .collect::<HashSet<usize>>();
                        if new_boxes.is_empty() {
                            for b in boxes_to_move {
                                boxes[b].row -= 1;
                            }
                            robot.0 -= 1;
                            break;
                        } else {
                            boxes_to_move.extend(new_boxes);
                        }
                    }
                }
            }
            'v' => {
                if !walls.contains(&(robot.0 + 1, robot.1)) {
                    let mut boxes_to_move = boxes.iter()
                        .enumerate()
                        .filter(|(_, b)| b.is_occupying(robot.0 + 1, robot.1))
                        .map(|(i, _)| i)
                        .collect::<HashSet<usize>>();
                    loop {
                        let check_coordinates = boxes_to_move.iter()
                            .map(|&i| &boxes[i])
                            .flat_map(|b| vec![(b.row + 1, b.col), (b.row + 1, b.col + 1)])
                            .collect::<HashSet<(usize, usize)>>();
                        if walls.iter().any(|&w| check_coordinates.contains(&w)) {
                            break;
                        }
                        let new_boxes = boxes.iter()
                            .enumerate()
                            .filter(|(_, b)| check_coordinates.iter().any(|&c| b.is_occupying(c.0, c.1)))
                            .map(|(i, _)| i)
                            .filter(|&i| !boxes_to_move.contains(&i))
                            .collect::<HashSet<usize>>();
                        if new_boxes.is_empty() {
                            for b in boxes_to_move {
                                boxes[b].row += 1;
                            }
                            robot.0 += 1;
                            break;
                        } else {
                            boxes_to_move.extend(new_boxes);
                        }
                    }
                }
            }
            '<' => {
                if !walls.contains(&(robot.0, robot.1 - 1)) {
                    let mut boxes_to_move = boxes.iter()
                        .enumerate()
                        .filter(|(_, b)| b.is_occupying(robot.0, robot.1 - 1))
                        .map(|(i, _)| i)
                        .collect::<HashSet<usize>>();
                    loop {
                        let check_coordinates = boxes_to_move.iter()
                            .map(|&i| &boxes[i])
                            .flat_map(|b| vec![(b.row, b.col - 1), (b.row, b.col - 1 + 1)])
                            .collect::<HashSet<(usize, usize)>>();
                        if walls.iter().any(|&w| check_coordinates.contains(&w)) {
                            break;
                        }
                        let new_boxes = boxes.iter()
                            .enumerate()
                            .filter(|(_, b)| check_coordinates.iter().any(|&c| b.is_occupying(c.0, c.1)))
                            .map(|(i, _)| i)
                            .filter(|&i| !boxes_to_move.contains(&i))
                            .collect::<HashSet<usize>>();
                        if new_boxes.is_empty() {
                            for b in boxes_to_move {
                                boxes[b].col -= 1;
                            }
                            robot.1 -= 1;
                            break;
                        } else {
                            boxes_to_move.extend(new_boxes);
                        }
                    }
                }
            }
            '>' => {
                if !walls.contains(&(robot.0, robot.1 + 1)) {
                    let mut boxes_to_move = boxes.iter()
                        .enumerate()
                        .filter(|(_, b)| b.is_occupying(robot.0, robot.1 + 1))
                        .map(|(i, _)| i)
                        .collect::<HashSet<usize>>();
                    loop {
                        let check_coordinates = boxes_to_move.iter()
                            .map(|&i| &boxes[i])
                            .flat_map(|b| vec![(b.row, b.col + 1), (b.row, b.col + 1 + 1)])
                            .collect::<HashSet<(usize, usize)>>();
                        if walls.iter().any(|&w| check_coordinates.contains(&w)) {
                            break;
                        }
                        let new_boxes = boxes.iter()
                            .enumerate()
                            .filter(|(_, b)| check_coordinates.iter().any(|&c| b.is_occupying(c.0, c.1)))
                            .map(|(i, _)| i)
                            .filter(|&i| !boxes_to_move.contains(&i))
                            .collect::<HashSet<usize>>();
                        if new_boxes.is_empty() {
                            for b in boxes_to_move {
                                boxes[b].col += 1;
                            }
                            robot.1 += 1;
                            break;
                        } else {
                            boxes_to_move.extend(new_boxes);
                        }
                    }
                }
            }
            '\n' => {}
            _ => {
                println!("Unknown instruction: {}", instruction);
            }
        }
    }
    result = boxes.iter().map(|b| b.row * 100 + b.col).sum();
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
