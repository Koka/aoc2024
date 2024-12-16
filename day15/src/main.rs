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

fn print_map(wh_map: &Vec<Vec<char>>) {
    let w = wh_map[0].len() + 2;

    println!();

    for _ in 0..w {
        print!("#");
    }
    println!();

    for row in wh_map {
        print!("#");
        print!("{}", row.iter().collect::<String>());
        println!("#");
    }

    for _ in 0..w {
        print!("#");
    }
    println!();

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

fn shift_rocks(wh_map: &mut Vec<Vec<char>>, rock: (usize, usize), dir: char) -> bool {
    let (j, i) = rock;

    let rock = match dir {
        '^' => {
            let whats_there = if j == 0 { '#' } else { wh_map[j - 1][i] };

            match whats_there {
                '.' => (j - 1, i),
                '#' => (j, i),
                'O' => {
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
                'O' => {
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
                'O' => {
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

            println!("Move left to {}", whats_there);
            match whats_there {
                '.' => (j, i - 1),
                '#' => (j, i),
                'O' => {
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
    wh_map[rock.0][rock.1] = 'O';

    rock != (j, i)
}

fn move_bot(wh_map: &mut Vec<Vec<char>>, bot: (usize, usize), dir: char) -> (usize, usize) {
    let (j, i) = bot;

    let bot = match dir {
        '^' => {
            let whats_there = if j == 0 { '#' } else { wh_map[j - 1][i] };

            println!("Move up to {}", whats_there);
            match whats_there {
                '.' => (j - 1, i),
                '#' => (j, i),
                'O' => {
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

            println!("Move right to {}", whats_there);
            match whats_there {
                '.' => (j, i + 1),
                '#' => (j, i),
                'O' => {
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

            println!("Move down to {}", whats_there);
            match whats_there {
                '.' => (j + 1, i),
                '#' => (j, i),
                'O' => {
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

            println!("Move left to {}", whats_there);
            match whats_there {
                '.' => (j, i - 1),
                '#' => (j, i),
                'O' => {
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

fn gps_sum(wh_map: &Vec<Vec<char>>) -> usize {
    wh_map.iter().enumerate().fold(0, |acc, (j, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (i, c)| {
            acc + match c {
                'O' => 100 * (j + 1) + i + 1,
                _ => 0,
            }
        })
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;
    let (mut wh_map, program) = parse_input(&input);

    print_map(&wh_map);
    println!("Program: {}", program.iter().collect::<String>());

    if let Some(mut bot) = find_bot(&wh_map) {
        for c in program {
            bot = move_bot(&mut wh_map, bot, c);
        }
    }

    print_map(&wh_map);

    println!("Part 1: {}", gps_sum(&wh_map));

    Ok(())
}
