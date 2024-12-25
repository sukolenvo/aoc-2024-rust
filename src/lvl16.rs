use std::collections::{HashMap, HashSet, VecDeque};

static WALL: i32 = -1;
static DIRECTION_EAST: (i32, i32) = (0, 1);
static DIRECTION_WEST: (i32, i32) = (0, -1);
static DIRECTION_NORTH: (i32, i32) = (-1, 0);
static DIRECTION_SOUTH: (i32, i32) = (1, 0);

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl16.txt");
    let map = Map::new(input);
    let pos = (map.data.len() - 2, 1);
    assert_eq!(map.data[pos.0][pos.1], 0);
    let end = (1, map.data[0].len() - 2);
    assert_eq!(map.data[end.0][end.1], 0);
    let mut paths = HashMap::new();
    paths.insert((pos, DIRECTION_EAST), 0);
    let mut queue = VecDeque::new();
    queue.push_back((pos, DIRECTION_EAST));
    while !queue.is_empty() {
        let (pos, direction) = queue.pop_front().unwrap();
        let clockwise = if direction == DIRECTION_EAST {
            DIRECTION_SOUTH
        } else if direction == DIRECTION_SOUTH {
            DIRECTION_WEST
        } else if direction == DIRECTION_WEST {
            DIRECTION_NORTH
        } else {
            DIRECTION_EAST
        };
        let counter_clockwise = if direction == DIRECTION_EAST {
            DIRECTION_NORTH
        } else if direction == DIRECTION_SOUTH {
            DIRECTION_EAST
        } else if direction == DIRECTION_WEST {
            DIRECTION_SOUTH
        } else {
            DIRECTION_WEST
        };
        for (next_pos, next_direction, cost) in vec![
            (
                (
                    (pos.0 as i32 + direction.0) as usize,
                    (pos.1 as i32 + direction.1) as usize,
                ),
                direction,
                1,
            ),
            (pos, clockwise, 1000),
            (pos, counter_clockwise, 1000),
        ] {
            if map.data[next_pos.0][next_pos.1] != WALL {
                if let Some(&val) = paths.get(&(next_pos, next_direction)) {
                    if (val > paths[&(pos, direction)] + cost) {
                        paths.insert((next_pos, next_direction), paths[&(pos, direction)] + cost);
                        queue.push_back((next_pos, next_direction));
                    }
                } else {
                    paths.insert((next_pos, next_direction), paths[&(pos, direction)] + cost);
                    queue.push_back((next_pos, next_direction));
                }
            }
        }
    }
    result = vec![
        paths[&(end, DIRECTION_EAST)],
        paths[&(end, DIRECTION_WEST)],
        paths[&(end, DIRECTION_NORTH)],
        paths[&(end, DIRECTION_SOUTH)],
    ]
    .iter()
    .min()
    .unwrap()
    .clone();
    println!("Result = {}", result);
}

struct Map {
    data: Vec<Vec<i32>>,
}

impl Map {
    fn new(input: &str) -> Self {
        Self {
            data: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => 0,
                            '#' => WALL,
                            'S' => 0,
                            'E' => 0,
                            _ => panic!("Unknown object"),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

fn part2() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl16.txt");
    let map = Map::new(input);
    let pos = (map.data.len() - 2, 1);
    assert_eq!(map.data[pos.0][pos.1], 0);
    let end = (1, map.data[0].len() - 2);
    assert_eq!(map.data[end.0][end.1], 0);
    let mut paths = HashMap::new();
    paths.insert((pos, DIRECTION_EAST), 0);
    let mut queue = VecDeque::new();
    queue.push_back((pos, DIRECTION_EAST));
    while !queue.is_empty() {
        let (pos, direction) = queue.pop_front().unwrap();
        let clockwise = if direction == DIRECTION_EAST {
            DIRECTION_SOUTH
        } else if direction == DIRECTION_SOUTH {
            DIRECTION_WEST
        } else if direction == DIRECTION_WEST {
            DIRECTION_NORTH
        } else {
            DIRECTION_EAST
        };
        let counter_clockwise = if direction == DIRECTION_EAST {
            DIRECTION_NORTH
        } else if direction == DIRECTION_SOUTH {
            DIRECTION_EAST
        } else if direction == DIRECTION_WEST {
            DIRECTION_SOUTH
        } else {
            DIRECTION_WEST
        };
        for (next_pos, next_direction, cost) in vec![
            (
                (
                    (pos.0 as i32 + direction.0) as usize,
                    (pos.1 as i32 + direction.1) as usize,
                ),
                direction,
                1,
            ),
            (pos, clockwise, 1000),
            (pos, counter_clockwise, 1000),
        ] {
            if map.data[next_pos.0][next_pos.1] != WALL {
                if let Some(&val) = paths.get(&(next_pos, next_direction)) {
                    if (val > paths[&(pos, direction)] + cost) {
                        paths.insert((next_pos, next_direction), paths[&(pos, direction)] + cost);
                        queue.push_back((next_pos, next_direction));
                    }
                } else {
                    paths.insert((next_pos, next_direction), paths[&(pos, direction)] + cost);
                    queue.push_back((next_pos, next_direction));
                }
            }
        }
    }
    let min_cost = vec![
        paths[&(end, DIRECTION_EAST)],
        paths[&(end, DIRECTION_WEST)],
        paths[&(end, DIRECTION_NORTH)],
        paths[&(end, DIRECTION_SOUTH)],
    ]
    .iter()
    .min()
    .unwrap()
    .clone();
    result = vec![
        (end, DIRECTION_EAST),
        (end, DIRECTION_WEST),
        (end, DIRECTION_NORTH),
        (end, DIRECTION_SOUTH),
    ]
    .iter()
    .filter(|&x| paths[&x] == min_cost)
    .map(|&(pos, direction)| find_tiles(&paths, &pos, &direction, min_cost).len())
    .sum();

    println!("Result = {}", result);
}

fn find_tiles(
    paths: &HashMap<((usize, usize), (i32, i32)), i32>,
    pos: &(usize, usize),
    direction: &(i32, i32),
    target_cost: i32,
) -> HashSet<(usize, usize)> {
    if paths.get(&(*pos, *direction)).unwrap_or(&0) != &target_cost {
        return HashSet::new();
    }
    let mut result = HashSet::new();
    result.insert(pos.clone());
    if target_cost > 1 {
        result.extend(find_tiles(
            paths,
            &(
                (pos.0 as i32 - direction.0) as usize,
                (pos.1 as i32 - direction.1) as usize,
            ),
            direction,
            target_cost - 1,
        ));
    }
    if target_cost >= 1000 {
        let clockwise = if *direction == DIRECTION_EAST {
            DIRECTION_SOUTH
        } else if *direction == DIRECTION_SOUTH {
            DIRECTION_WEST
        } else if *direction == DIRECTION_WEST {
            DIRECTION_NORTH
        } else {
            DIRECTION_EAST
        };
        let counter_clockwise = if *direction == DIRECTION_EAST {
            DIRECTION_NORTH
        } else if *direction == DIRECTION_SOUTH {
            DIRECTION_EAST
        } else if *direction == DIRECTION_WEST {
            DIRECTION_SOUTH
        } else {
            DIRECTION_WEST
        };
        result.extend(find_tiles(paths, pos, &clockwise, target_cost - 1000));
        result.extend(find_tiles(paths, pos, &counter_clockwise, target_cost - 1000));
    }
    result
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
