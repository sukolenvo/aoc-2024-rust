use std::collections::HashSet;

fn part1() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl20.txt");
    let size = input.lines().count();
    let mut tiles = vec![vec![false; size]; size];
    let lines = input.lines().collect::<Vec<&str>>();
    for r in 0..lines.len() {
        let line = lines[r];
        for c in 0..line.len() {
            tiles[r][c] = line.chars().nth(c).unwrap() != '#';
        }
    }
    let start = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains('S'))
        .map(|(r, line)| (r, line.find('S').unwrap()))
        .next()
        .unwrap();
    let end = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains('E'))
        .map(|(r, line)| (r, line.find('E').unwrap()))
        .next()
        .unwrap();
    let solver = Solver {};
    let path = solver.solve(&tiles, start, end);
    let optimal_path = path.len();
    assert_eq!(optimal_path, solver.solve_simple(&tiles, start, end));
    let mut cheats = HashSet::new();
    for tile in path {
        if !tiles[tile.0 - 1][tile.1] {
            cheats.insert((tile.0 - 1, tile.1));
        }
        if !tiles[tile.0 + 1][tile.1] {
            cheats.insert((tile.0 + 1, tile.1));
        }
        if !tiles[tile.0][tile.1 - 1] {
            cheats.insert((tile.0, tile.1 - 1));
        }
        if !tiles[tile.0][tile.1 + 1] {
            cheats.insert((tile.0, tile.1 + 1));
        }
    }
    println!("Optimal path = {}", optimal_path);
    println!("Cheats = {}", cheats.len());
    for i in 0..cheats.len() {
        let cheat = cheats.iter().nth(i).unwrap();
        tiles[cheat.0][cheat.1] = true;
        let path = solver.solve_simple(&tiles, start, end);
        if optimal_path - path >= 100 {
            result += 1;
            println!("Cheat {} = {:?} - {}", i, cheat, optimal_path - path);
        }
        tiles[cheat.0][cheat.1] = false;
    }
    println!("Result = {}", result);
}

struct Solver {}

impl Solver {
    fn solve(
        &self,
        tiles: &Vec<Vec<bool>>,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut visited = vec![vec![vec![]; tiles[0].len()]; tiles.len()];
        let mut queue = vec![];
        queue.push(start);
        while !queue.is_empty() {
            let (r, c) = queue.remove(0);
            if r == end.0 && c == end.1 {
                return visited[r][c].clone();
            }
            if r > 0 && tiles[r - 1][c] && visited[r - 1][c].is_empty() {
                queue.push((r - 1, c));
                let mut val = visited[r][c].clone();
                val.push((r - 1, c));
                visited[r - 1][c] = val;
            }
            if r < tiles.len() - 1 && tiles[r + 1][c] && visited[r + 1][c].is_empty() {
                queue.push((r + 1, c));
                let mut val = visited[r][c].clone();
                val.push((r + 1, c));
                visited[r + 1][c] = val;
            }
            if c > 0 && tiles[r][c - 1] && visited[r][c - 1].is_empty() {
                queue.push((r, c - 1));
                let mut val = visited[r][c].clone();
                val.push((r, c - 1));
                visited[r][c - 1] = val;
            }
            if c < tiles[0].len() - 1 && tiles[r][c + 1] && visited[r][c + 1].is_empty() {
                queue.push((r, c + 1));
                let mut val = visited[r][c].clone();
                val.push((r, c + 1));
                visited[r][c + 1] = val;
            }
        }
        panic!("No path found");
    }

    fn solve_simple(
        &self,
        tiles: &Vec<Vec<bool>>,
        start: (usize, usize),
        end: (usize, usize),
    ) -> usize {
        let mut visited = HashSet::new();
        let mut queue = vec![];
        queue.push((start.0, start.1, 0));
        while !queue.is_empty() {
            let (r, c, d) = queue.remove(0);
            if r == end.0 && c == end.1 {
                return d;
            }
            if visited.contains(&(r, c)) {
                continue;
            }
            visited.insert((r, c));
            if r > 0 && tiles[r - 1][c] {
                queue.push((r - 1, c, d + 1));
            }
            if r < tiles.len() - 1 && tiles[r + 1][c] {
                queue.push((r + 1, c, d + 1));
            }
            if c > 0 && tiles[r][c - 1] {
                queue.push((r, c - 1, d + 1));
            }
            if c < tiles[0].len() - 1 && tiles[r][c + 1] {
                queue.push((r, c + 1, d + 1));
            }
        }
        panic!("No path found");
    }
}

fn part2() {
    let mut result = 0;
    let input = include_str!("../tasks/lvl20.txt");
    let size = input.lines().count();
    let mut tiles = vec![vec![false; size]; size];
    let lines = input.lines().collect::<Vec<&str>>();
    for r in 0..lines.len() {
        let line = lines[r];
        for c in 0..line.len() {
            tiles[r][c] = line.chars().nth(c).unwrap() != '#';
        }
    }
    let start = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains('S'))
        .map(|(r, line)| (r, line.find('S').unwrap()))
        .next()
        .unwrap();
    let end = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains('E'))
        .map(|(r, line)| (r, line.find('E').unwrap()))
        .next()
        .unwrap();

    let end_distances = get_distances_map(&tiles, end);
    let mut cheat_jumps: HashSet<(i32, i32, usize)> = HashSet::new();
    for i in 0..=20 {
        for j in 0..=(20 - i) {
            cheat_jumps.insert((i, j, (i + j) as usize));
            cheat_jumps.insert((-i, j, (i + j) as usize));
            cheat_jumps.insert((i, -j, (i + j) as usize));
            cheat_jumps.insert((-i, -j, (i + j) as usize));
        }
    }
    let start_distances = get_distances_map(&tiles, start);
    for r in 0..size {
        for c in 0..size {
            if tiles[r][c] {
                for &(dr, dc, t) in &cheat_jumps {
                    let end_r = r as i32 + dr;
                    let end_c = c as i32 + dc;
                    if end_r >= 0
                        && end_r < size as i32
                        && end_c >= 0
                        && end_c < size as i32
                        && tiles[end_r as usize][end_c as usize]
                    {
                        let path = start_distances[r][c] + t + end_distances[end_r as usize][end_c as usize];
                        // if path == 84-72 {
                        if path <= 9484-100 {
                        //     println!("{} {}", dr, dc);
                        //     println!("{} {} {}", start_distances[r][c], t, end_distances[end_r as usize][end_c as usize]);
                        //     println!("Cheat {:?} to {:?} = {}", (r, c), (end_r, end_c), 84 - path);
                            result += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Result = {}", result);
}

fn get_distances_map(tiles: &Vec<Vec<bool>>, from: (usize, usize)) -> Vec<Vec<usize>> {
    let mut visited = vec![vec![0; tiles[0].len()]; tiles.len()];
    let mut queue = vec![];
    queue.push(from);
    while !queue.is_empty() {
        let (r, c) = queue.remove(0);
        if r > 0 && tiles[r - 1][c] && visited[r - 1][c] == 0 {
            queue.push((r - 1, c));
            visited[r - 1][c] = visited[r][c] + 1;
        }
        if r < tiles.len() - 1 && tiles[r + 1][c] && visited[r + 1][c] == 0 {
            queue.push((r + 1, c));
            visited[r + 1][c] = visited[r][c] + 1;
        }
        if c > 0 && tiles[r][c - 1] && visited[r][c - 1] == 0 {
            queue.push((r, c - 1));
            visited[r][c - 1] = visited[r][c] + 1;
        }
        if c < tiles[0].len() - 1 && tiles[r][c + 1] && visited[r][c + 1] == 0 {
            queue.push((r, c + 1));
            visited[r][c + 1] = visited[r][c] + 1;
        }
    }
    visited[from.0][from.1] = 0;
    visited
}

struct Solver2 {}

impl Solver2 {}

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
