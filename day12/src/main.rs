use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

fn find_regions(plot_map: &Vec<Vec<char>>) -> Vec<(char, usize, usize, usize)> {
    let mut regions = vec![];

    let mut visited = HashSet::new();

    for i in 0..plot_map.len() {
        for j in 0..plot_map[i].len() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let start = plot_map[i][j];
            if start != '.' {
                let mut area = 0usize;
                let mut perimeter = 0usize;
                let mut sides = 0usize;

                let mut stack = vec![(i, j)];
                while let Some((i, j)) = stack.pop() {
                    if visited.contains(&(i, j)) {
                        continue;
                    }

                    visited.insert((i, j));
                    area += 1;

                    for di in -1..=1 {
                        for dj in -1..=1 {
                            if (di == 0 && dj == 0) || !(di == 0 || dj == 0) {
                                continue;
                            }
                            let next_i = i as isize + di;
                            let next_j = j as isize + dj;

                            if next_i < 0
                                || next_j < 0
                                || next_i >= plot_map.len() as isize
                                || next_j >= plot_map[next_i as usize].len() as isize
                            {
                                perimeter += 1;
                            } else {
                                let next_i = next_i as usize;
                                let next_j = next_j as usize;
                                let next = plot_map[next_i][next_j];

                                if next == start {
                                    stack.push((next_i, next_j));
                                } else {
                                    perimeter += 1;
                                }
                            }
                        }
                    }
                }

                regions.push((start, area, perimeter, sides));
            }
        }
    }

    regions
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input_simple.txt")?;

    let plot_map = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let regions = find_regions(&plot_map);

    println!(
        "Part 1: {}",
        regions.iter().fold(0, |acc, r| acc + r.1 * r.2)
    );

    println!(
        "Part 2: {}",
        regions.iter().fold(0, |acc, r| acc + r.1 * r.3)
    );

    Ok(())
}
