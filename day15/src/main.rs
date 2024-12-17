use std::{error::Error, fs};

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut wh_map = vec![];
    let mut program = vec![];
    let mut mode = "map";

    for s in input.lines().skip(1) {
        if s.is_empty() {
            wh_map.pop();
            mode = "program";
            continue;
        }
        match mode {
            "map" => {
                let mut chars = s.chars().skip(1).collect::<Vec<_>>();
                chars.pop();
                wh_map.push(chars);
            }
            "program" => {
                let mut chars = s.chars().collect::<Vec<_>>();
                program.append(&mut chars);
            }
            _ => {
                unreachable!("Invalid mode")
            }
        }
    }

    (wh_map, program)
}

fn print_map(wh_map: &Vec<Vec<char>>, scaled: bool) {
    let w = wh_map[0].len() + 2;

    println!();

    print!("{}", if scaled { "#" } else { "" });
    for _ in 0..w {
        print!("#");
    }
    println!("{}", if scaled { "#" } else { "" });

    for row in wh_map {
        print!("{}", if scaled { "##" } else { "#" });
        print!("{}", row.iter().collect::<String>());
        println!("{}", if scaled { "##" } else { "#" });
    }

    print!("{}", if scaled { "#" } else { "" });
    for _ in 0..w {
        print!("#");
    }
    println!("{}", if scaled { "#" } else { "" });

    println!();
}

fn find_bot(wh_map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (j, row) in wh_map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == '@' {
                return Some((j, i));
            }
        }
    }

    None
}

fn can_shift_rocks(wh_map: &Vec<Vec<char>>, rock: (usize, usize), dir: char) -> bool {
    let (j, i) = rock;

    let mut rock_parts = vec![];
    match wh_map[j][i] {
        'O' => rock_parts.push((j, i)),
        '[' => {
            rock_parts.push((j, i));
            rock_parts.push((j, i + 1));
        }
        ']' => {
            rock_parts.push((j, i - 1));
            rock_parts.push((j, i));
        }
        _ => unreachable!("Bad rock"),
    };

    match dir {
        '>' => {
            let &(j, i) = rock_parts.last().expect("No rock parts");

            let whats_there = if i == wh_map[j].len() - 1 {
                '#'
            } else {
                wh_map[j][i + 1]
            };

            match whats_there {
                '.' => true,
                '#' => false,
                'O' | '[' => can_shift_rocks(wh_map, (j, i + 1), dir),
                _ => unreachable!("Strange thing happened"),
            }
        }
        '<' => {
            let &(j, i) = rock_parts.first().expect("No rock parts");

            let whats_there = if i == 0 { '#' } else { wh_map[j][i - 1] };

            match whats_there {
                '.' => true,
                '#' => false,
                'O' | ']' => can_shift_rocks(wh_map, (j, i - 1), dir),
                _ => unreachable!("Strange thing happened"),
            }
        }
        '^' => rock_parts.iter().all(|&(j, i)| {
            let whats_there = if j == 0 { '#' } else { wh_map[j - 1][i] };

            match whats_there {
                '.' => true,
                '#' => false,
                'O' | ']' | '[' => can_shift_rocks(wh_map, (j - 1, i), dir),
                _ => unreachable!("Strange thing happened"),
            }
        }),
        'v' => rock_parts.iter().all(|&(j, i)| {
            let whats_there = if j == wh_map.len() - 1 {
                '#'
            } else {
                wh_map[j + 1][i]
            };

            match whats_there {
                '.' => true,
                '#' => false,
                'O' | ']' | '[' => can_shift_rocks(wh_map, (j + 1, i), dir),
                _ => unreachable!("Strange thing happened"),
            }
        }),
        _ => unreachable!("Bad direction"),
    }
}

fn shift_rocks(wh_map: &mut Vec<Vec<char>>, rock: (usize, usize), dir: char) -> bool {
    let can_do = can_shift_rocks(wh_map, rock, dir);
    if !can_do {
        return false;
    }

    let (j, i) = rock;

    let mut rock_parts = vec![];
    match wh_map[j][i] {
        'O' => rock_parts.push((j, i)),
        '[' => {
            rock_parts.push((j, i));
            rock_parts.push((j, i + 1));
        }
        ']' => {
            rock_parts.push((j, i - 1));
            rock_parts.push((j, i));
        }
        _ => unreachable!("Bad rock"),
    };

    match dir {
        '>' => {
            let &(j, i) = rock_parts.last().expect("No rock parts");

            let whats_there = if i == wh_map[j].len() - 1 {
                '#'
            } else {
                wh_map[j][i + 1]
            };

            match whats_there {
                '.' => {
                    rock_parts.iter().rev().for_each(|&(j, i)| {
                        let next_sym = wh_map[j][i + 1];
                        wh_map[j][i + 1] = wh_map[j][i];
                        wh_map[j][i] = next_sym;
                    });
                }
                'O' | '[' => {
                    shift_rocks(wh_map, (j, i + 1), dir);

                    rock_parts.iter().rev().for_each(|&(j, i)| {
                        let next_sym = wh_map[j][i + 1];
                        wh_map[j][i + 1] = wh_map[j][i];
                        wh_map[j][i] = next_sym;
                    });
                }
                _ => unreachable!("Strange thing happened"),
            }
        }
        '<' => {
            let &(j, i) = rock_parts.first().expect("No rock parts");

            let whats_there = if i == 0 { '#' } else { wh_map[j][i - 1] };

            match whats_there {
                '.' => {
                    rock_parts.iter().for_each(|&(j, i)| {
                        let next_sym = wh_map[j][i - 1];
                        wh_map[j][i - 1] = wh_map[j][i];
                        wh_map[j][i] = next_sym;
                    });
                }
                'O' | ']' => {
                    shift_rocks(wh_map, (j, i - 1), dir);

                    rock_parts.iter().for_each(|&(j, i)| {
                        let next_sym = wh_map[j][i - 1];
                        wh_map[j][i - 1] = wh_map[j][i];
                        wh_map[j][i] = next_sym;
                    });
                }
                _ => unreachable!("Strange thing happened"),
            }
        }
        '^' => {
            rock_parts.iter().for_each(|&(j, i)| {
                let whats_there = if j == 0 { '#' } else { wh_map[j - 1][i] };

                match whats_there {
                    '.' => {
                        let next_sym = wh_map[j - 1][i];
                        wh_map[j - 1][i] = wh_map[j][i];
                        wh_map[j][i] = next_sym;
                    }
                    'O' | ']' | '[' => {
                        shift_rocks(wh_map, (j - 1, i), dir);

                        let next_sym = wh_map[j - 1][i];
                        wh_map[j - 1][i] = wh_map[j][i];
                        wh_map[j][i] = next_sym;
                    }
                    _ => unreachable!("Strange thing happened"),
                }
            });
        }
        'v' => {
            rock_parts.iter().for_each(|&(j, i)| {
                let whats_there = if j == wh_map.len() - 1 {
                    '#'
                } else {
                    wh_map[j + 1][i]
                };

                match whats_there {
                    '.' => {
                        let next_sym = wh_map[j + 1][i];
                        wh_map[j + 1][i] = wh_map[j][i];
                        wh_map[j][i] = next_sym;
                    }
                    'O' | ']' | '[' => {
                        shift_rocks(wh_map, (j + 1, i), dir);

                        let next_sym = wh_map[j + 1][i];
                        wh_map[j + 1][i] = wh_map[j][i];
                        wh_map[j][i] = next_sym;
                    }
                    _ => unreachable!("Strange thing happened"),
                }
            });
        }
        _ => unreachable!("Bad direction"),
    };

    true
}

fn move_bot(wh_map: &mut Vec<Vec<char>>, bot: (usize, usize), dir: char) -> (usize, usize) {
    let (j, i) = bot;

    let bot = match dir {
        '^' => {
            let whats_there = if j == 0 { '#' } else { wh_map[j - 1][i] };

            match whats_there {
                '.' => (j - 1, i),
                '#' => (j, i),
                'O' | '[' | ']' => {
                    if shift_rocks(wh_map, (j - 1, i), dir) {
                        (j - 1, i)
                    } else {
                        (j, i)
                    }
                }
                _ => (j, i),
            }
        }
        '>' => {
            let whats_there = if i == wh_map[j].len() - 1 {
                '#'
            } else {
                wh_map[j][i + 1]
            };

            match whats_there {
                '.' => (j, i + 1),
                '#' => (j, i),
                'O' | '[' | ']' => {
                    if shift_rocks(wh_map, (j, i + 1), dir) {
                        (j, i + 1)
                    } else {
                        (j, i)
                    }
                }
                _ => (j, i),
            }
        }
        'v' => {
            let whats_there = if j == wh_map.len() - 1 {
                '#'
            } else {
                wh_map[j + 1][i]
            };

            match whats_there {
                '.' => (j + 1, i),
                '#' => (j, i),
                'O' | '[' | ']' => {
                    if shift_rocks(wh_map, (j + 1, i), dir) {
                        (j + 1, i)
                    } else {
                        (j, i)
                    }
                }
                _ => (j, i),
            }
        }
        '<' => {
            let whats_there = if i == 0 { '#' } else { wh_map[j][i - 1] };

            match whats_there {
                '.' => (j, i - 1),
                '#' => (j, i),
                'O' | '[' | ']' => {
                    if shift_rocks(wh_map, (j, i - 1), dir) {
                        (j, i - 1)
                    } else {
                        (j, i)
                    }
                }
                _ => (j, i),
            }
        }
        _ => {
            unreachable!("Unkown move");
        }
    };

    wh_map[j][i] = '.';
    wh_map[bot.0][bot.1] = '@';

    bot
}

fn gps_sum(wh_map: &Vec<Vec<char>>, scaled: bool) -> usize {
    wh_map.iter().enumerate().fold(0, |acc, (j, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (i, c)| {
            acc + match c {
                'O' | '[' => 100 * (j + 1) + i + if scaled { 2 } else { 1 },
                _ => 0,
            }
        })
    })
}

fn scale_map(wh_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut scaled = vec![];

    for row in wh_map {
        let mut scaled_row = vec![];
        for c in row {
            match c {
                '@' => {
                    scaled_row.push('@');
                    scaled_row.push('.');
                }
                'O' => {
                    scaled_row.push('[');
                    scaled_row.push(']');
                }
                c => {
                    scaled_row.push(*c);
                    scaled_row.push(*c);
                }
            }
        }
        scaled.push(scaled_row);
    }

    scaled
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;
    let (orig_map, program) = parse_input(&input);

    println!("Program: {}", program.iter().collect::<String>());

    let mut wh_map = orig_map.clone();

    print_map(&wh_map, false);

    if let Some(mut bot) = find_bot(&wh_map) {
        for c in &program {
            bot = move_bot(&mut wh_map, bot, *c);
        }
    }

    print_map(&wh_map, false);

    println!("Part 1: {}", gps_sum(&wh_map, false));

    let mut wh_map = scale_map(&orig_map);

    print_map(&wh_map, true);

    if let Some(mut bot) = find_bot(&wh_map) {
        for c in &program {
            bot = move_bot(&mut wh_map, bot, *c);
        }
    }

    print_map(&wh_map, true);

    println!("Part 2: {}", gps_sum(&wh_map, true));

    Ok(())
}
