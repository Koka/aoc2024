use std::{collections::HashMap, error::Error, fs};

fn deep_blink(n: usize, blink_count: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if blink_count == 0 {
        return 1;
    }

    if let Some(&x) = memo.get(&(n, blink_count)) {
        return x;
    }

    if n == 0 {
        let x = deep_blink(1, blink_count - 1, memo);
        memo.insert((1, blink_count - 1), x);

        return x;
    }

    let digits = n.to_string();
    if digits.len() % 2 == 0 {
        let (l, r) = digits.split_at(digits.len() / 2);
        let l = l.parse::<usize>().expect("Left is NaN");
        let r = r.parse::<usize>().expect("Right is NaN");

        let x = deep_blink(l, blink_count - 1, memo);
        memo.insert((l, blink_count - 1), x);

        let y = deep_blink(r, blink_count - 1, memo);
        memo.insert((r, blink_count - 1), y);

        return x + y;
    }

    let x = deep_blink(n * 2024, blink_count - 1, memo);
    memo.insert((n * 2024, blink_count - 1), x);

    x
}

fn recursive_blinking(stones: &Vec<usize>, blink_count: usize) -> usize {
    let mut memo = HashMap::new();

    stones
        .iter()
        .map(|s| deep_blink(*s, blink_count, &mut memo))
        .sum::<usize>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let stones = input
        .split(' ')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("Part 1: {}", recursive_blinking(&stones, 25));

    println!("Part 2: {}", recursive_blinking(&stones, 75));

    Ok(())
}
