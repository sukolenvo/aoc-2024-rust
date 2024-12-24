use crate::common::read_task;

fn part1() {
    let input = read_task(9);
    let mut files = input
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let gaps = input
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, c)| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let mut result = 0;
    let mut pos = 0;
    let mut file_idx = 0;
    let mut back_file_idx = files.len();
    let mut back_file = (0u32, 0u32);
    loop {
        if file_idx == back_file_idx {
            break;
        }
        for _ in 0..files[file_idx] {
            result += file_idx * pos;
            pos += 1;
        }
        for _ in 0..gaps[file_idx] {
            if back_file.1 > 0 {
                back_file.1 -= 1;
                result += back_file.0 as usize * pos;
                pos += 1;
            } else if (back_file_idx > file_idx + 1) {
                back_file_idx -= 1;
                let file = files[back_file_idx];
                back_file = (back_file_idx as u32, file - 1);
                result += back_file.0 as usize * pos;
                pos += 1;
            } else {
                break;
            }
        }
        file_idx += 1;
    }
    for _ in 0..back_file.1 {
        result += back_file.0 as usize * pos;
        pos += 1;
    }
    println!("Result: {}", result);
}

struct File {
    id: usize,
    len: usize,
    pos: usize,
}

struct Gap {
    len: usize,
    pos: usize,
}

impl File {
    fn checksum(&self) -> usize {
        let mut result = 0;
        for i in 0..self.len {
            result += self.id * (self.pos + i);
        }
        result
    }
}

fn part2() {
    let input = read_task(9);
    let mut result = 0;
    let mut pos = 0;
    let mut files = Vec::new();
    let mut gaps = Vec::new();
    for (i, char) in input.chars().enumerate() {
        let val = char.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push(File {
                id: i / 2,
                len: val,
                pos,
            });
        } else {
            gaps.push(Gap { len: val, pos });
        }
        pos += val;
    }
    for i in (0..files.len()).rev() {
        let file = files.get_mut(i).unwrap();
        if let Some(gap) = gaps.iter_mut().find(|gap| gap.len >= file.len && gap.pos < file.pos) {
            file.pos = gap.pos;
            gap.pos += file.len;
            gap.len -= file.len;
        }
    }
    result = files.iter().map(|file| file.checksum()).sum();
    // files.sort_by(|a, b| a.pos.cmp(&b.pos));
    // for i in 0..files.len() {
    //     for _ in 0..files[i].len {
    //         print!("{} ", files[i].id);
    //     }
    //     if i + 1 < files.len() {
    //         for _ in 0..files[i+1].pos - files[i].pos - files[i].len {
    //             print!(". ");
    //         }
    //     }
    // }
    println!("\nResult: {}", result);
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
        part2(); //8504654861152
    }
}
