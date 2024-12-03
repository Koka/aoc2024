use std::{error::Error, fs};

use regex::Regex;

fn scan(program: &str, check_cond: bool) -> u32 {
    let re_scan =
        Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").expect("Invalid scan regex");

    let mut result = 0u32;
    let mut ignored = false;

    for (_, [cmd]) in re_scan.captures_iter(program).map(|c| c.extract()) {
        match cmd {
            "do()" => {
                ignored = false;
            }
            "don't()" => {
                ignored = true;
            }
            s => {
                if !ignored || !check_cond {
                    let parts = s.split_once(",");
                    if let Some((l, r)) = parts {
                        if let Some((_, l)) = l.split_once("(") {
                            if let Some((r, _)) = r.split_once(")") {
                                let l: u32 = l.parse().expect("NAN");
                                let r: u32 = r.parse().expect("NAN");
                                result += l * r;
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("./input.txt")?;

    println!("Part 1: {}", scan(&program, false));
    println!("Part 2: {}", scan(&program, true));

    Ok(())
}
