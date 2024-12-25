use crate::common::read_task;

fn part1() {
    let input = read_task(13);
    let arcades = input.split("\n\n")
        .map(|x| x.trim())
        .map(|x| Arcade::new(x))
        .collect::<Vec<Arcade>>();
    let mut result = 0;
    for i in 0..arcades.len() {
        match arcades[i].find_ab() {
            Ok((a, b)) => {
                println!("{:?}: a: {}, b: {}", i, a, b);
                if a <= 100 && b <= 100 {
                    result += 3 * a + b;
                }
            },
            Err(_) => (),
        }
    }
    println!("Result: {}", result);
}

struct Arcade {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    prize_x: i64,
    prize_y: i64,
}

impl Arcade {
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    fn new(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].starts_with("Button A: X+"));
        assert!(lines[1].starts_with("Button B: X+"));
        assert!(lines[2].starts_with("Prize: X="));
        let a = lines[0]["Button A: X+".len()..].split(", Y+").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let b = lines[1]["Button B: X+".len()..].split(", Y+").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let prize = lines[2]["Prize: X=".len()..].split(", Y=").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 2);
        assert_eq!(prize.len(), 2);
        Self {
            ax: a[0],
            ay: a[1],
            bx: b[0],
            by: b[1],
            prize_x: prize[0],
            prize_y: prize[1],
        }
    }

    fn find_ab(&self) -> Result<(u64, u64), ()> {
        // a*ax + b*bx = prize_x
        // a*ay + b*by = prize_y
        // then
        // a * ax * ay + b * bx * ay = prize_x * ay
        // a * ay * ax + b * by * ax = prize_y * ax
        // then
        // b = (prize_x * ay - prize_y * ax) / (bx * ay - by * ax)
        let b = (self.prize_x * self.ay - self.prize_y * self.ax) / (self.bx * self.ay - self.by * self.ax);
        let a = (self.prize_x - b * self.bx) / self.ax;
        if (a * self.ax + b * self.bx) == self.prize_x && (a * self.ay + b * self.by) == self.prize_y {
            Ok((a as u64, b as u64))
        } else {
            Err(())
        }

    }
}

fn part2() {
    let input = read_task(13);
    let mut arcades = input.split("\n\n")
        .map(|x| x.trim())
        .map(|x| Arcade::new(x))
        .collect::<Vec<Arcade>>();
    for mut arcade in arcades.iter_mut() {
        arcade.prize_x += 10000000000000;
        arcade.prize_y += 10000000000000;
    }
    let mut result = 0;
    for i in 0..arcades.len() {
        match arcades[i].find_ab() {
            Ok((a, b)) => {
                println!("{:?}: a: {}, b: {}", i, a, b);
                result += 3 * a + b;
            },
            Err(_) => (),
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