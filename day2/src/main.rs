use std::{error::Error, fs};

fn check_safe(arr: &[u32]) -> bool {
    let increasing: bool = arr[0] < arr[1];
    let mut safe = true;
    for i in 0..arr.len() - 1 {
        safe =
            safe && if increasing {
                arr[i + 1] > arr[i]
            } else {
                arr[i + 1] < arr[i]
            } && (arr[i + 1]).abs_diff(arr[i]) <= 3;

        if !safe {
            break;
        }
    }
    safe
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let (safe_count, soft_safe_count) = input
        .lines()
        .map(|rep| {
            rep.split(" ")
                .filter_map(|lev| lev.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .fold((0u32, 0u32), |acc, rep| {
            let n = rep.len();

            let safe = check_safe(&rep);

            let mut soft_safe = safe;
            if !soft_safe {
                for i in 0..n {
                    let mut dup = rep.clone();
                    dup.remove(i);

                    let dup_safe = check_safe(&dup);
                    if dup_safe {
                        soft_safe = true;
                        break;
                    }
                }
            }

            (
                acc.0 + if safe { 1 } else { 0 },
                acc.1 + if soft_safe { 1 } else { 0 },
            )
        });

    println!("Part 1: {}", safe_count);
    println!("Part 2: {}", soft_safe_count);

    Ok(())
}
