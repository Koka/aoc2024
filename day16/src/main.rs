use std::{collections::HashMap, error::Error, fs};

fn print_maze(maze: &Vec<Vec<char>>) {
    println!();
    for row in maze {
        for c in row {
            print!("{}", *c);
        }
        println!();
    }
    println!();
}

fn find_tile(maze: &Vec<Vec<char>>, tile: char) -> Option<(usize, usize)> {
    maze.iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(i, &c)| if c == tile { Some((j, i)) } else { None })
        })
        .next()
}

fn path_score(path: &[char]) -> usize {
    path.iter().fold(0, |acc, c| {
        acc + match &c {
            '+' => 1,
            _ => 1000,
        }
    })
}

fn print_path(maze: &Vec<Vec<char>>, path: &Vec<char>, initial_state: ((usize, usize), char)) {
    let mut my_maze = maze.clone();

    let (mut pos, mut dir) = initial_state;
    my_maze[pos.0][pos.1] = ' ';

    for action in path {
        match action {
            '+' => {
                pos = match dir {
                    '>' => (pos.0, pos.1 + 1),
                    '^' => (pos.0 - 1, pos.1),
                    '<' => (pos.0, pos.1 - 1),
                    'v' => (pos.0 + 1, pos.1),
                    _ => unreachable!("Bad dir"),
                };
            }
            '<' => {
                dir = match dir {
                    '>' => '^',
                    '^' => '<',
                    '<' => 'v',
                    'v' => '>',
                    _ => unreachable!("Bad dir"),
                };
            }
            '>' => {
                dir = match dir {
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    '^' => '>',
                    _ => unreachable!("Bad dir"),
                };
            }
            _ => unreachable!("Bad action"),
        }
        my_maze[pos.0][pos.1] = ' ';
    }

    println!("Score: {}", path_score(path));
    print_maze(&my_maze);
}

fn find_paths(
    maze: &Vec<Vec<char>>,
    start: (usize, usize),
    finish: (usize, usize),
    dir: char,
) -> Vec<Vec<char>> {
    let mut found_paths = vec![];

    let mut visited = HashMap::new();
    let mut queue = vec![((start, dir), vec![])];

    while let Some((state, path)) = queue.pop() {
        let (pos, dir) = state;

        if pos == finish {
            found_paths.push(path);
            continue;
        }

        if let Some(&old_score) = visited.get(&state) {
            if old_score <= path_score(&path) {
                continue;
            }
        }

        visited.insert(state, path_score(&path));

        // + is move, < is rotate left, > is rotate right
        for op in ['+', '<', '>'] {
            match op {
                '+' => {
                    let new_pos = match dir {
                        '>' => (pos.0, pos.1 + 1),
                        '^' => (pos.0 - 1, pos.1),
                        '<' => (pos.0, pos.1 - 1),
                        'v' => (pos.0 + 1, pos.1),
                        _ => unreachable!("Bad dir"),
                    };

                    let whats_there = maze[new_pos.0][new_pos.1];

                    if whats_there != '#' {
                        let mut path = path.clone();
                        path.push(op);
                        queue.insert(0, ((new_pos, dir), path))
                    }
                }
                '<' => {
                    let new_dir = match dir {
                        '>' => '^',
                        '^' => '<',
                        '<' => 'v',
                        'v' => '>',
                        _ => unreachable!("Bad dir"),
                    };

                    let mut path = path.clone();
                    path.push(op);
                    queue.insert(0, ((pos, new_dir), path))
                }
                '>' => {
                    let new_dir = match dir {
                        '>' => 'v',
                        'v' => '<',
                        '<' => '^',
                        '^' => '>',
                        _ => unreachable!("Bad dir"),
                    };

                    let mut path = path.clone();
                    path.push(op);
                    queue.insert(0, ((pos, new_dir), path))
                }
                _ => unreachable!("Bad op"),
            }
        }
    }

    found_paths
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let maze = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if let Some(start) = find_tile(&maze, 'S') {
        if let Some(finish) = find_tile(&maze, 'E') {
            let dir = '>';

            print_maze(&maze);

            println!("Start: {:?}", &start);
            println!("Finish: {:?}", &finish);
            println!("Direction: {:?}", &dir);

            let found_paths = find_paths(&maze, start, finish, dir);

            println!();

            println!("Paths found: {}", found_paths.len());
            // for p in &found_paths {
            //     print_path(&maze, p, (start, dir));
            // }

            let min_score = found_paths
                .iter()
                .map(|p| path_score(p))
                .min()
                .expect("No paths found");

            println!("Part 1: {}", min_score)
        }
    }

    Ok(())
}
