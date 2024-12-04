use std::{error::Error, fs};

enum Direction {
    Horizontal,
    Vertical,
    DiagLeft,
    DiagRight,
}

fn count(haystack: &str, needle: &str, dir: &Direction) -> u32 {
    let matrix = haystack
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let w = matrix[0].len();
    let h = matrix.len();

    match &dir {
        Direction::Horizontal => {
            let mut count = 0;

            for j in 0..h {
                for i in 0..=w - needle.len() {
                    let mut found = true;
                    for k in 0..needle.len() {
                        let a = &needle.chars().nth(k).expect("Needle overflow");
                        let b = &matrix[j][i + k];

                        found = found && *a == *b
                    }
                    if found {
                        count += 1;
                    }
                }
            }

            count
        }
        Direction::Vertical => {
            let mut count = 0;

            for j in 0..=h - needle.len() {
                for i in 0..w {
                    let mut found = true;
                    for k in 0..needle.len() {
                        let a = &needle.chars().nth(k).expect("Needle overflow");
                        let b = &matrix[j + k][i];

                        found = found && *a == *b
                    }
                    if found {
                        count += 1;
                    }
                }
            }

            count
        }
        Direction::DiagLeft => {
            let mut count = 0;

            for j in 0..=h - needle.len() {
                for i in 0..=w - needle.len() {
                    let mut found = true;
                    for k in 0..needle.len() {
                        let a = &needle.chars().nth(k).expect("Needle overflow");
                        let b = &matrix[j + k][i + k];

                        found = found && *a == *b
                    }
                    if found {
                        count += 1;
                    }
                }
            }

            count
        }
        Direction::DiagRight => {
            let mut count = 0;

            for j in 0..=h - needle.len() {
                for i in 0..=w - needle.len() {
                    let mut found = true;
                    for k in 0..needle.len() {
                        let a = &needle.chars().nth(k).expect("Needle overflow");
                        let b = &matrix[j + k][w - (i + k) - 1];

                        found = found && *a == *b
                    }
                    if found {
                        count += 1;
                    }
                }
            }

            count
        }
    }
}

fn part1(haystack: &str) {
    let needle = "XMAS";
    let rev_needle = &needle.chars().rev().collect::<String>();

    let mut total = 0;

    for dir in [
        Direction::Horizontal,
        Direction::Vertical,
        Direction::DiagLeft,
        Direction::DiagRight,
    ] {
        total += count(&haystack, needle, &dir);
        total += count(&haystack, rev_needle, &dir);
    }

    println!("Part 1: {}", total);
}

fn gen_patterns() -> Vec<[[char; 3]; 3]> {
    let needle = "MAS";
    let rev_needle = &needle.chars().rev().collect::<String>();

    let mut patterns = vec![];
    for a_rev in 0..=1 {
        for b_rev in 0..=1 {
            let mut pattern = [['.', '.', '.'], ['.', '.', '.'], ['.', '.', '.']];

            let a = if a_rev == 1 { rev_needle } else { needle };
            let b = if b_rev == 1 { rev_needle } else { needle };

            for i in 0..a.len() {
                pattern[i][i] = a.chars().nth(i).expect("Char overflow");
            }

            for i in 0..b.len() {
                pattern[i][pattern[i].len() - i - 1] = b.chars().nth(i).expect("Char overflow");
            }

            patterns.push(pattern);
        }
    }

    patterns
}

fn count_pattern(haystack: &str, pattern: &[[char; 3]; 3]) -> u32 {
    let matrix = haystack
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let w = matrix[0].len();
    let h = matrix.len();

    let mut count = 0;

    for j in 0..=h - pattern.len() {
        for i in 0..=w - pattern[0].len() {
            let mut found = true;

            for wj in 0..pattern.len() {
                for wi in 0..pattern[0].len() {
                    let w = pattern[wj][wi];
                    let c = matrix[j + wj][i + wi];

                    found = found && (w == '.' || w == c);
                }
            }

            if found {
                count += 1;
            }
        }
    }

    count
}

fn part2(haystack: &str) {
    let patterns = gen_patterns();

    let mut total = 0;

    for pattern in patterns {
        total += count_pattern(haystack, &pattern);
    }

    println!("Part 2: {}", total);
}

fn main() -> Result<(), Box<dyn Error>> {
    let haystack = fs::read_to_string("./input.txt")?;

    part1(&haystack);
    part2(&haystack);

    Ok(())
}
