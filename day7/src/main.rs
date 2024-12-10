use std::{collections::HashMap, error::Error, fs};

use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Ops {
    Add,
    Multiply,
    Concat,
}

fn is_valid(
    result: usize,
    nums: &[usize],
    ops: &[Ops],
    cache: &mut HashMap<usize, Vec<Vec<Ops>>>,
) -> bool {
    if nums.is_empty() {
        return result == 0;
    } else if nums.len() == 1 {
        return result == nums[0];
    }

    let k = nums.len() - 1;
    let cached = cache.get(&k);
    let all_progs = match cached {
        Some(v) => v.clone(),
        None => {
            // println!("Gen progs L = {}", k);
            let all_progs = ops
                .iter()
                .combinations_with_replacement(nums.len() - 1)
                .flat_map(|p| {
                    p.clone()
                        .into_iter()
                        .permutations(p.len())
                        .unique()
                        .map(|op| op.into_iter().copied().collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            cache.insert(k, all_progs.clone());

            // println!("Total progs L = {} :: {}", k, all_progs.len());

            all_progs
        }
    };

    for sub_p in all_progs {
        let mut acc = nums[0];
        for (i, op) in sub_p.iter().enumerate() {
            match op {
                Ops::Add => acc += nums[i + 1],
                Ops::Multiply => acc *= nums[i + 1],
                Ops::Concat => {
                    acc = (acc.to_string() + &nums[i + 1].to_string())
                        .parse::<usize>()
                        .unwrap();
                }
            }
        }

        if acc == result {
            return true;
        }
    }

    false
}

fn valid_sum(data: &[(usize, Vec<usize>)], ops: &[Ops]) -> usize {
    let mut cache = HashMap::new();

    let valid_sum = data
        .iter()
        .filter(|&(r, nums)| is_valid(*r, nums, ops, &mut cache))
        .fold(0, |acc, it| acc + it.0);

    valid_sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let data = input
        .lines()
        .filter_map(|s| {
            let (result, num_str) = s.split_once(": ")?;

            let result = result.parse::<usize>().ok()?;
            let nums = num_str
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();

            Some((result, nums))
        })
        .collect::<Vec<_>>();

    // println!("Part 1: {}", valid_sum(&data, &[Ops::Add, Ops::Multiply]));

    println!(
        "Part 2: {}",
        valid_sum(&data, &[Ops::Add, Ops::Multiply, Ops::Concat])
    );

    Ok(())
}
