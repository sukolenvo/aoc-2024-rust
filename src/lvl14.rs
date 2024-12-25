use std::io;
use crate::common::read_task;
use std::io::{Read, Write};

fn part1() {
    let mut result = 0;
    let input = read_task(14);
    let robots = input.lines().map(|x| Robot::new(x)).collect::<Vec<Robot>>();
    let width = 101;
    let height = 103;
    let rounds = 100;
    let width_middle = width / 2;
    let height_middle = height / 2;
    let mut quadrants = vec![0, 0, 0, 0];
    for robot in &robots {
        let mut new_x = (robot.x + robot.vx * rounds) % width;
        let mut new_y = (robot.y + robot.vy * rounds) % height;
        if new_x < 0 {
            new_x += width;
        }
        if new_y < 0 {
            new_y += height;
        }
        if new_x == width_middle || new_y == height_middle {
            continue;
        }
        let q_idx =
            if new_x < width_middle { 0 } else { 1 } + if new_y < height_middle { 0 } else { 2 };
        quadrants[q_idx] += 1;
    }
    result = quadrants.iter().product::<i32>();
    println!("Result: {}", result);
}

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn new(input: &str) -> Self {
        let pos_velocity = input.split(" ").collect::<Vec<&str>>();
        assert_eq!(pos_velocity.len(), 2);
        let x_y = pos_velocity[0].split("=").last().unwrap();
        let x = x_y.split(",").next().unwrap().parse::<i32>().unwrap();
        let y = x_y.split(",").last().unwrap().parse::<i32>().unwrap();
        let vx_vy = pos_velocity[1].split("=").last().unwrap();
        let vx = vx_vy.split(",").next().unwrap().parse::<i32>().unwrap();
        let vy = vx_vy.split(",").last().unwrap().parse::<i32>().unwrap();
        Self { x, y, vx, vy }
    }
}

fn part2() {
    let mut result = 0;
    let input = read_task(14);
    let mut robots = input.lines().map(|x| Robot::new(x)).collect::<Vec<Robot>>();
    let width = 101;
    let height = 103;
    let mut area = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(0);
        }
        area.push(row);
    }
    let mut t = 1;
    loop {
        for r in 0..height {
            for c in 0..width {
                area[r][c] = 0;
            }
        }
        for mut robot in robots.iter_mut() {
            robot.x = (robot.x + robot.vx) % width as i32;
            robot.y = (robot.y + robot.vy) % height as i32;
            if robot.x < 0 {
                robot.x += width as i32;
            }
            if robot.y < 0 {
                robot.y += height as i32;
            }
            area[robot.y as usize][robot.x as usize] += 1;
        }
        let mut diff = 0;
        // if left part is right
        for r in 0..height {
            for c in 0..width / 2 {
                if area[r][c] != area[r][width - 1 - c] {
                    diff += 1;
                }
            }
        }
        if (t - 27) % 101 == 0 {
            for r in 0..height {
                for c in 0..width {
                    print!("{}", if area[r][c] > 0 { "#" } else { "." });
                }
                println!();
            }
            println!("Time: {}", t);
            let mut buffer = [0; 1];
            io::stdin().read_exact(&mut buffer).unwrap();
        }
        t += 1;
        if t % 1000 == 0 {
            println!("Time: {}", t);
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
