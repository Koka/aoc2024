use std::{error::Error, fs, iter::zip};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut left = vec![];
    let mut right = vec![];

    input
        .lines()
        .filter_map(|l| {
            l.split_once(' ')
                .map(|(a, b)| (a.trim().parse::<u32>(), b.trim().parse::<u32>()))
        })
        .for_each(|(a, b)| {
            left.push(a.expect("Number expected"));
            right.push(b.expect("Number expected"));
        });

    left.sort();
    right.sort();

    let zipped = zip(&left, &right);
    let mut sum = 0u32;

    for (a, b) in zipped {
        sum += a.abs_diff(*b);
    }

    println!("Part 1: {}", sum);

    let mut score = 0u32;

    for a in left {
        let idx = right.binary_search(&a);
        let mut mul = 0u32;
        if let Ok(idx) = idx {
            let mut i = idx;
            let mut j = idx;

            while i > 0 && right[i] == a {
                if right[i - 1] == a {
                    i -= 1;
                } else {
                    break;
                }
            }

            while j < right.len() - 1 && right[j] == a {
                if right[j + 1] == a {
                    j += 1;
                } else {
                    break;
                }
            }

            mul = (j - i + 1) as u32;
        }
        score += a * mul;
    }

    println!("Part 2: {}", score);

    Ok(())
}
