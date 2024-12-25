use crate::common::read_task;
use std::collections::HashMap;

fn part1() {
    let input = read_task(12);
    let farm = Farm::new(&input);
    let mut regions: Vec<Region> = Vec::new();
    let mut result = 0;
    for r in 0..farm.plots.len() {
        for c in 0..farm.plots[r].len() {
            if regions.iter().any(|x| x.includes_plot((r, c))) {
                continue;
            }
            regions.push(build_region(&farm, r, c));
        }
    }
    for region in regions {
        println!(
            "{:?}: {}x{} {:?}",
            region.letter,
            region.get_area(),
            region.get_perimeter(farm.plots[0].len(), farm.plots.len()),
            region.plots
        );
        result += region.get_area() * region.get_perimeter(farm.plots[0].len(), farm.plots.len());
    }
    println!("Result: {}", result);
}

fn build_region(farm: &Farm, r: usize, c: usize) -> Region {
    let plot = farm.plots[r][c];
    let mut plots = vec![(r, c)];
    let mut i = 0;
    while i < plots.len() {
        let (r, c) = plots[i];
        if r > 0 && farm.plots[r - 1][c] == plot {
            if !plots.contains(&(r - 1, c)) {
                plots.push((r - 1, c));
            }
        }
        if c > 0 && farm.plots[r][c - 1] == plot {
            if !plots.contains(&(r, c - 1)) {
                plots.push((r, c - 1));
            }
        }
        if r < farm.plots.len() - 1 && farm.plots[r + 1][c] == plot {
            if !plots.contains(&(r + 1, c)) {
                plots.push((r + 1, c));
            }
        }
        if c < farm.plots[r].len() - 1 && farm.plots[r][c + 1] == plot {
            if !plots.contains(&(r, c + 1)) {
                plots.push((r, c + 1));
            }
        }
        i += 1;
    }
    Region {
        letter: plot,
        plots,
    }
}

struct Farm {
    plots: Vec<Vec<char>>,
}

impl Farm {
    fn new(input: &String) -> Farm {
        let plots = input.split("\n").map(|x| x.chars().collect()).collect();
        Farm { plots }
    }
}

struct Region {
    letter: char,
    plots: Vec<(usize, usize)>,
}

impl Region {
    fn includes_plot(&self, plot: (usize, usize)) -> bool {
        self.plots.contains(&plot)
    }

    fn get_area(&self) -> usize {
        self.plots.len()
    }

    fn get_perimeter(&self, farm_width: usize, farm_height: usize) -> usize {
        self.get_perimeter_edges(farm_width, farm_height).len()
    }

    fn get_perimeter_edges(&self, farm_width: usize, farm_height: usize) -> Vec<Fence> {
        let mut result = Vec::new();
        for &(r, c) in &self.plots {
            if r == 0 || !self.plots.contains(&(r - 1, c)) {
                result.push(Fence {
                    r1: r,
                    c1: c,
                    r2: r,
                    c2: c + 1,
                });
            }
            if c == 0 || !self.plots.contains(&(r, c - 1)) {
                result.push(Fence {
                    r1: r,
                    c1: c,
                    r2: r + 1,
                    c2: c,
                });
            }
            if r == farm_height - 1 || !self.plots.contains(&(r + 1, c)) {
                result.push(Fence {
                    r1: r + 1,
                    c1: c,
                    r2: r + 1,
                    c2: c + 1,
                });
            }
            if c == farm_width - 1 || !self.plots.contains(&(r, c + 1)) {
                result.push(Fence {
                    r1: r,
                    c1: c + 1,
                    r2: r + 1,
                    c2: c + 1,
                });
            }
        }
        result
    }

    fn get_sides(&self, farm_width: usize, farm_height: usize) -> usize {
        let edges = self.get_perimeter_edges(farm_width, farm_height);
        let mut neighbors = 0;
        for edge in &edges {
            for other in &edges {
                if edge.is_adjacent(other) {
                    neighbors += 1;
                }
            }
        }
        let mut points = HashMap::new();
        for f in &edges {
            *points.entry((f.r1, f.c1)).or_insert(0) += 1;
            *points.entry((f.r2, f.c2)).or_insert(0) += 1;
        }
        edges.len() - neighbors / 2 + points.iter().filter(|&(_, &count)| count == 4).count() * 2
    }
}

struct Fence {
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
}

impl Fence {
    fn is_horizontal(&self) -> bool {
        self.r1 == self.r2
    }

    fn is_adjacent(&self, other: &Fence) -> bool {
        self.r1 == self.r2
            && other.r1 == other.r2
            && self.r1 == other.r1
            && (self.c1 == other.c2 || self.c2 == other.c1)
            || self.c1 == self.c2
                && other.c1 == other.c2
                && self.c1 == other.c1
                && (self.r1 == other.r2 || self.r2 == other.r1)
    }
}

fn part2() {
    let input = read_task(12);
    let farm = Farm::new(&input);
    let mut regions: Vec<Region> = Vec::new();
    let mut result = 0;
    for r in 0..farm.plots.len() {
        for c in 0..farm.plots[r].len() {
            if regions.iter().any(|x| x.includes_plot((r, c))) {
                continue;
            }
            regions.push(build_region(&farm, r, c));
        }
    }
    for region in regions {
        println!(
            "{:?}: {}x{} {:?}",
            region.letter,
            region.get_area(),
            region.get_sides(farm.plots[0].len(), farm.plots.len()),
            region.plots
        );
        result += region.get_area() * region.get_sides(farm.plots[0].len(), farm.plots.len());
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
