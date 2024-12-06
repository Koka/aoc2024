use std::{collections::HashSet, error::Error, fs};

fn traverse(matrix: &[Vec<char>]) -> (HashSet<(isize, isize)>, bool) {
    let w = matrix[0].len();
    let h = matrix.len();

    let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
    let mut guard = (0isize, 0isize, '^');

    for j in 0..h {
        for i in 0..w {
            if matrix[j][i] == '#' || matrix[j][i] == 'O' {
                obstacles.insert((j.try_into().unwrap(), i.try_into().unwrap()));
            } else if matrix[j][i] != '.' {
                guard = (j.try_into().unwrap(), i.try_into().unwrap(), matrix[j][i]);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut looping_for = 0;
    while looping_for < w * h {
        if visited.contains(&(guard.0, guard.1)) {
            looping_for += 1;
        } else {
            visited.insert((guard.0, guard.1));
            looping_for = 0;
        }

        match guard.2 {
            '^' => {
                if obstacles.contains(&(guard.0 - 1, guard.1)) {
                    guard.2 = '>';
                } else {
                    guard.0 -= 1;
                }
            }
            'v' => {
                if obstacles.contains(&(guard.0 + 1, guard.1)) {
                    guard.2 = '<';
                } else {
                    guard.0 += 1;
                }
            }
            '<' => {
                if obstacles.contains(&(guard.0, guard.1 - 1)) {
                    guard.2 = '^';
                } else {
                    guard.1 -= 1;
                }
            }
            '>' => {
                if obstacles.contains(&(guard.0, guard.1 + 1)) {
                    guard.2 = 'v';
                } else {
                    guard.1 += 1;
                }
            }
            _ => unreachable!(),
        }
        if guard.0 < 0
            || guard.0 >= h.try_into().unwrap()
            || guard.1 < 0
            || guard.1 >= w.try_into().unwrap()
        {
            break;
        }
    }

    (visited, looping_for > 0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let matrix = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (visited, _looped) = traverse(&matrix);

    println!("Part 1: {}", visited.len());

    let vj_min = visited
        .iter()
        .min_by(|a, b| a.0.cmp(&b.0))
        .map(|x| x.0)
        .unwrap();

    let vj_max = visited
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|x| x.0)
        .unwrap();

    let vi_min = visited
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|x| x.1)
        .unwrap();

    let vi_max = visited
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|x| x.1)
        .unwrap();

    let w = matrix[0].len();
    let h = matrix.len();

    let mut obstacle_positions = HashSet::new();

    for j in vj_min - 1..=vj_max + 1 {
        for i in vi_min - 1..=vi_max + 1 {
            let j = if j < 0 {
                0
            } else if j > (h - 1).try_into().unwrap() {
                h - 1
            } else {
                j as usize
            };
            let i = if i < 0 {
                0
            } else if i > (w - 1).try_into().unwrap() {
                w - 1
            } else {
                i as usize
            };

            if matrix[j][i] == '.' {
                let mut candidate = matrix.clone();
                candidate[j][i] = 'O';

                let (_visited, looped) = traverse(&candidate);

                if looped {
                    obstacle_positions.insert((j, i));
                }
            }
        }
    }

    println!("Part 2: {}", obstacle_positions.len());

    Ok(())
}
