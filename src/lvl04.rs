use crate::common::read_task;

pub fn lvl04() {
    let input = read_task(4);
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        for j in 0..line.len() {
            result += count_xmas(&lines, i, j);
        }
    }
    println!("Result: {}", result);
}

fn count_xmas(lines: &Vec<&str>, i: usize, j: usize) -> i32 {
    if lines[i].chars().nth(j).unwrap() != 'X' {
        return 0;
    }
    let mut found = 0;
    if lines[i].len() > j + 3 // right
        && lines[i].chars().nth(j + 1).unwrap() == 'M'
        && lines[i].chars().nth(j + 2).unwrap() == 'A'
        && lines[i].chars().nth(j + 3).unwrap() == 'S'
    {
        found += 1;
    }
    if j >= 3 // left
        && lines[i].chars().nth(j - 1).unwrap() == 'M'
        && lines[i].chars().nth(j - 2).unwrap() == 'A'
        && lines[i].chars().nth(j - 3).unwrap() == 'S'
    {
        found += 1;
    }
    // down
    if i + 3 < lines.len()
        && lines[i + 1].chars().nth(j).unwrap() == 'M'
        && lines[i + 2].chars().nth(j).unwrap() == 'A'
        && lines[i + 3].chars().nth(j).unwrap() == 'S'
    {
        found += 1;
    }
    //up
    if i >= 3
        && lines[i - 1].chars().nth(j).unwrap() == 'M'
        && lines[i - 2].chars().nth(j).unwrap() == 'A'
        && lines[i - 3].chars().nth(j).unwrap() == 'S'
    {
        found += 1;
    }
    // diagonal right down
    if i + 3 < lines.len()
        && lines[i].len() > j + 3
        && lines[i + 1].chars().nth(j + 1).unwrap() == 'M'
        && lines[i + 2].chars().nth(j + 2).unwrap() == 'A'
        && lines[i + 3].chars().nth(j + 3).unwrap() == 'S'
    {
        found += 1;
    }
    // diagonal left down
    if i + 3 < lines.len()
        && j >= 3
        && lines[i + 1].chars().nth(j - 1).unwrap() == 'M'
        && lines[i + 2].chars().nth(j - 2).unwrap() == 'A'
        && lines[i + 3].chars().nth(j - 3).unwrap() == 'S'
    {
        found += 1;
    }
    // diagonal right up
    if i >= 3
        && lines[i].len() > j + 3
        && lines[i - 1].chars().nth(j + 1).unwrap() == 'M'
        && lines[i - 2].chars().nth(j + 2).unwrap() == 'A'
        && lines[i - 3].chars().nth(j + 3).unwrap() == 'S'
    {
        found += 1;
    }
    // diagonal left up
    if i >= 3
        && j >= 3
        && lines[i - 1].chars().nth(j - 1).unwrap() == 'M'
        && lines[i - 2].chars().nth(j - 2).unwrap() == 'A'
        && lines[i - 3].chars().nth(j - 3).unwrap() == 'S'
    {
        found += 1;
    }
    found
}

pub fn lvl04_second() {
    let input = read_task(4);
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        for j in 0..line.len() {
            result += count_x_mas(&lines, i, j);
        }
    }
    println!("Result: {}", result);
}

fn count_x_mas(lines: &Vec<&str>, i: usize, j: usize) -> i32 {
    if lines[i].chars().nth(j).unwrap() != 'A' {
        return 0;
    }
    if i > 0 && j > 0 && i < lines.len() - 1 && j < lines[i].len() - 1 {
        let top_left = lines[i - 1].chars().nth(j - 1).unwrap();
        let top_right = lines[i - 1].chars().nth(j + 1).unwrap();
        let bottom_left = lines[i + 1].chars().nth(j - 1).unwrap();
        let bottom_right = lines[i + 1].chars().nth(j + 1).unwrap();
        if ((top_left, bottom_right) == ('M', 'S') || (top_left, bottom_right) == ('S', 'M'))
            && ((top_right, bottom_left) == ('M', 'S') || (top_right, bottom_left) == ('S', 'M'))
        {
            return 1;
        }
    }
    0
}
