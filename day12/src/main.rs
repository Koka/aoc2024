use std::{collections::HashSet, error::Error, fs};

fn find_regions(plot_map: &Vec<Vec<char>>) -> Vec<(char, usize, usize, usize)> {
    let mut regions = vec![];

    let mut visited = HashSet::new();

    for i in 0..plot_map.len() {
        for j in 0..plot_map[i].len() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let start = plot_map[i][j];
            let mut cells = HashSet::new();

            if start != '.' {
                let mut area = 0usize;
                let mut perimeter = 0usize;

                let mut stack = vec![(i, j)];
                while let Some((i, j)) = stack.pop() {
                    if visited.contains(&(i, j)) {
                        continue;
                    }

                    visited.insert((i, j));
                    cells.insert((i, j));
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

                let mut corners = 0;

                for (i, j) in cells {
                    let mut window = [[None; 3]; 3];

                    for di in -1..=1 {
                        for dj in -1..=1 {
                            let next_i = i as isize + di;
                            let next_j = j as isize + dj;
                            if next_i >= 0
                                && next_i < plot_map.len() as isize
                                && next_j >= 0
                                && next_j < plot_map[i].len() as isize
                            {
                                let it = plot_map[next_i as usize][next_j as usize];

                                window[(di + 1) as usize][(dj + 1) as usize] =
                                    if it == start { Some(it) } else { None };
                            }
                        }
                    }

                    if window[0][1] == window[2][1]
                        && window[1][0] == window[1][2]
                        && window[0][1] != window[1][0]
                    {
                        continue;
                    }

                    let mut neighbour_count = 0usize;
                    for di in 0..window.len() {
                        for dj in 0..window[di].len() {
                            if (di == 1 && dj == 1) || !(di == 1 || dj == 1) {
                                continue;
                            }

                            if window[di][dj].is_some() {
                                neighbour_count += 1;
                            }
                        }
                    }

                    let my_corners = match neighbour_count {
                        0 => 4,
                        1 => 2,
                        2 => {
                            let is_outer = (
                                // T & R
                                window[0][1].is_some()
                                    && window[1][2].is_some()
                                    && window[0][2].is_some()
                            ) || (
                                // R & B
                                window[1][2].is_some()
                                    && window[2][1].is_some()
                                    && window[2][2].is_some()
                            ) || (
                                // B & L
                                window[2][1].is_some()
                                    && window[1][0].is_some()
                                    && window[2][0].is_some()
                            ) || (
                                // L & T
                                window[1][0].is_some()
                                    && window[0][1].is_some()
                                    && window[0][0].is_some()
                            );

                            if is_outer {
                                1
                            } else {
                                2
                            }
                        }
                        3 => {
                            let is_horizontal = window[1][0].is_some() && window[1][2].is_some();

                            let mut diag_count = 0;

                            if is_horizontal {
                                let search_i = if window[0][1].is_some() { 0 } else { 2 };
                                for dj in [0, 2] {
                                    diag_count += if window[search_i][dj].is_none() { 1 } else { 0 }
                                }
                            } else {
                                let search_j = if window[1][0].is_some() { 0 } else { 2 };
                                for di in [0, 2] {
                                    diag_count += if window[di][search_j].is_none() { 1 } else { 0 }
                                }
                            }

                            diag_count
                        }
                        4 => {
                            let mut diag_count = 0;
                            for di in [0, 2] {
                                for dj in [0, 2] {
                                    diag_count += if window[di][dj].is_none() { 1 } else { 0 }
                                }
                            }
                            diag_count
                        }
                        _ => 0,
                    };

                    corners += my_corners;

                    // let s = window
                    //     .iter()
                    //     .map(|r| r.iter().map(|c| c.unwrap_or('.')).collect::<String>() + "\n")
                    //     .collect::<String>();

                    // println!("({}, {}) = {}", i, j, my_corners);
                    // println!("{}", s);
                }

                // println!("Region {} has {} corners", start, corners);

                regions.push((start, area, perimeter, corners));
            }
        }
    }

    regions
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

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
