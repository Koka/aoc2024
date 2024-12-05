use std::{cmp::Ordering, collections::HashMap, error::Error, fs};

fn is_correct(upd: &[String], rules: &[(&str, &str)]) -> bool {
    let mut lookup = HashMap::new();
    for (i, p) in upd.iter().enumerate() {
        lookup.insert(p.clone(), i);
    }

    let mut is_correct = true;

    for (a, b) in rules {
        let a_idx = lookup.get(*a);
        let b_idx = lookup.get(*b);

        let ordered = match (a_idx, b_idx) {
            (Some(&a_idx), Some(&b_idx)) => a_idx < b_idx,
            _ => true,
        };

        is_correct = is_correct && ordered;

        if !is_correct {
            break;
        }
    }

    is_correct
}

fn part1(updates: &[Vec<&str>], rules: &[(&str, &str)]) {
    let mut mid_sum = 0u32;

    for u in updates {
        let clone = u.iter().map(|&s| s.to_owned()).collect::<Vec<_>>();
        if is_correct(&clone, rules) {
            mid_sum += u[u.len() / 2].parse::<u32>().expect("Invalid number");
        }
    }

    println!("Part 1: {}", mid_sum);
}

fn reorder(upd: Vec<&str>, rules: &[(&str, &str)]) -> Vec<String> {
    let mut result = vec![];

    for p in upd {
        result.push(p.to_owned());
    }

    result.sort_by(|a, b| {
        for &(l, r) in rules {
            if l == a && r == b {
                return Ordering::Less;
            } else if r == a && l == b {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    });

    result
}

fn part2(updates: &[Vec<&str>], rules: &[(&str, &str)]) {
    let mut mid_sum = 0u32;

    for u in updates {
        let clone = u.iter().map(|&s| s.to_owned()).collect::<Vec<_>>();
        if !is_correct(&clone, rules) {
            let reordered = reorder(u.clone(), rules);

            if !is_correct(&reordered, rules) {
                panic!("Reorder failed!")
            }

            mid_sum += reordered[reordered.len() / 2]
                .parse::<u32>()
                .expect("Invalid number");
        }
    }

    println!("Part 2: {}", mid_sum);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut rules = vec![];

    let mut updates = vec![];

    for s in input.lines() {
        if let Some((a, b)) = s.split_once('|') {
            rules.push((a, b));
        } else if !s.is_empty() {
            updates.push(s.split(',').collect::<Vec<_>>());
        }
    }

    part1(&updates, &rules);
    part2(&updates, &rules);

    Ok(())
}
