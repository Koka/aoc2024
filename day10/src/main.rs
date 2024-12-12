use std::{collections::HashSet, error::Error, fs};

fn find_trails(
    row_idx: usize,
    cell_idx: usize,
    topo_map: &Vec<Vec<Option<u32>>>,
) -> HashSet<((usize, usize), Vec<(usize, usize)>)> {
    let mut trail_ends = HashSet::new();

    let mut stack: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![];
    stack.push(((row_idx, cell_idx), vec![(row_idx, cell_idx)]));

    while let Some(((i, j), path)) = stack.pop() {
        if let Some(me) = topo_map[i][j] {
            if me == 9 {
                trail_ends.insert(((i, j), path.clone()));
            }

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx == 0 && dy == 0) || !(dx == 0 || dy == 0) {
                        continue;
                    }

                    let next_i = i as isize + dy;
                    let next_j = j as isize + dx;

                    if next_i >= 0
                        && next_i < topo_map.len() as isize
                        && next_j >= 0
                        && next_j < topo_map[next_i as usize].len() as isize
                    {
                        let next_i = next_i as usize;
                        let next_j = next_j as usize;

                        if let Some(next) = topo_map[next_i][next_j] {
                            if next == me + 1 {
                                stack.push((
                                    (next_i, next_j),
                                    [path.clone(), vec![(next_i, next_j)]].concat(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    trail_ends
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let topo_map = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut score_sum = 0;
    let mut rating_sum = 0;

    for (i, row) in topo_map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == Some(0) {
                let trail_ends = find_trails(i, j, &topo_map);
                let rating = trail_ends.len();

                rating_sum += rating;

                let score = trail_ends
                    .iter()
                    .map(|end| end.0)
                    .collect::<HashSet<_>>()
                    .len();

                score_sum += score;
            }
        }
    }

    println!("Part 1: {}", score_sum);
    println!("Part 1: {}", rating_sum);

    Ok(())
}
