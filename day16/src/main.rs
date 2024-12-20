use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

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

fn collect_path(
    maze: &Vec<Vec<char>>,
    path: &Vec<char>,
    initial_state: ((usize, usize), char),
) -> HashSet<(usize, usize)> {
    let mut tiles = HashSet::new();

    let (mut pos, mut dir) = initial_state;
    tiles.insert(pos);

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
        tiles.insert(pos);
    }

    tiles
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

    let mut min_score = None;

    while let Some((state, path)) = queue.pop() {
        let (pos, dir) = state;

        let my_score = path_score(&path);

        if let Some(old_score) = min_score {
            if my_score > old_score {
                continue;
            }
        }

        if pos == finish {
            if let Some(old_score) = min_score {
                if my_score < old_score {
                    min_score = Some(my_score);
                }
            }

            if min_score.is_none() {
                min_score = Some(my_score);
            }

            found_paths.push(path);

            continue;
        }

        if let Some(&old_score) = visited.get(&state) {
            // TODO: < loops on real input, <= gives wrong part 2 result
            if old_score < path_score(&path) {
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
    let input = fs::read_to_string("./input_simple.txt")?;

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

            let min_score = found_paths
                .iter()
                .map(|p| path_score(p))
                .min()
                .expect("No paths found");

            println!("Part 1: {}", min_score);

            let min_points = found_paths
                .iter()
                .filter(|p| path_score(p) == min_score)
                .flat_map(|p| collect_path(&maze, p, (start, dir)))
                .collect::<HashSet<_>>();

            println!("Part 2: {}", min_points.len());
        }
    }

    Ok(())
}
