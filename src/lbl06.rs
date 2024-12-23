use crate::common::read_task;
use std::collections::HashSet;

fn part1() {
    let (mut map, mut guard) = parse_map();
    walk(&mut map, &mut guard);
    println!(
        "Number of visited cells: {}",
        map.data
            .iter()
            .flatten()
            .filter(|&x| *x == MapState::Visited)
            .count()
    );
}

fn walk(map: &mut Map, guard: &mut Guard) -> String {
    let mut visited = HashSet::new();
    loop {
        let res = step(map, guard);
        match res {
            Ok(_) => {
                if visited.contains(guard) {
                    return "Loop detected".to_string();
                }
                visited.insert(guard.clone());
                // println!("{}", msg);
            }
            Err(msg) => {
                println!("{}", msg);
                return msg;
            }
        }
    }
}

fn step(map: &mut Map, guard: &mut Guard) -> Result<String, String> {
    match guard.direction {
        Direction::Up => {
            if (guard.y == 0) {
                return Err("Guard is out of bounds".to_string());
            }
            if (map.data[guard.y - 1][guard.x] == MapState::Obstacle) {
                guard.direction = Direction::Right;
                Ok("Changed direction to Right".to_string())
            } else {
                guard.y -= 1;
                map.data[guard.y][guard.x] = MapState::Visited;
                Ok(format!("Moved to {} {}", guard.x, guard.y))
            }
        }
        Direction::Down => {
            if (guard.y == map.data.len() - 1) {
                return Err("Guard is out of bounds".to_string());
            }
            if (map.data[guard.y + 1][guard.x] == MapState::Obstacle) {
                guard.direction = Direction::Left;
                Ok("Changed direction to Left".to_string())
            } else {
                guard.y += 1;
                map.data[guard.y][guard.x] = MapState::Visited;
                Ok(format!("Moved to {} {}", guard.x, guard.y))
            }
        }
        Direction::Left => {
            if (guard.x == 0) {
                return Err("Guard is out of bounds".to_string());
            }
            if (map.data[guard.y][guard.x - 1] == MapState::Obstacle) {
                guard.direction = Direction::Up;
                Ok("Changed direction to Up".to_string())
            } else {
                guard.x -= 1;
                map.data[guard.y][guard.x] = MapState::Visited;
                Ok(format!("Moved to {} {}", guard.x, guard.y))
            }
        }
        Direction::Right => {
            if (guard.x == map.data[0].len() - 1) {
                return Err("Guard is out of bounds".to_string());
            }
            if (map.data[guard.y][guard.x + 1] == MapState::Obstacle) {
                guard.direction = Direction::Down;
                Ok("Changed direction to Down".to_string())
            } else {
                guard.x += 1;
                map.data[guard.y][guard.x] = MapState::Visited;
                Ok(format!("Moved to {} {}", guard.x, guard.y))
            }
        }
    }
}

#[derive(PartialEq, Clone)]
enum MapState {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Clone)]
struct Map {
    data: Vec<Vec<MapState>>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

fn parse_map() -> (Map, Guard) {
    let input = read_task(6);
    let lines = input.lines().collect::<Vec<&str>>();
    let map = Map {
        data: lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => MapState::Empty,
                        '^' => MapState::Visited,
                        '#' => MapState::Obstacle,
                        _ => panic!("Unexpected character"),
                    })
                    .collect()
            })
            .collect(),
    };

    let guard_position = lines
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.chars().position(|c| c == '^').map(|j| (i, j)))
        .expect("Guard position not found");

    let guard = Guard {
        x: guard_position.1,
        y: guard_position.0,
        direction: Direction::Up,
    };
    (map, guard)
}

fn part2() {
    let (map, guard) = parse_map();
    let mut result = 0;
    let mut path = map.clone();
    let mut path_guard = guard.clone();
    walk(&mut path, &mut path_guard);
    for i in 0..map.data.len() {
        for j in 0..map.data[i].len() {
            // println!("{} {}", i, j);
            if path.data[i][j] == MapState::Visited && (i, j) != (guard.y, guard.x) {
                let mut map = map.clone();
                let mut guard = guard.clone();
                map.data[i][j] = MapState::Obstacle;
                let res = walk(&mut map, &mut guard);
                if (res == "Loop detected") {
                    result += 1;
                }
            }
        }
    }

    println!("Number of loops: {}", result);
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
