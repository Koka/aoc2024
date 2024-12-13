use std::{error::Error, fs};

#[derive(Default, Debug)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

impl ClawMachine {
    fn calc_a_b(&self) -> Option<(usize, usize)> {
        let a_dx = self.button_a.0 as f64;
        let b_dx = self.button_b.0 as f64;
        let r_x = b_dx / a_dx;

        let a_dy = self.button_a.1 as f64;
        let b_dy = self.button_b.1 as f64;
        let r_y = b_dy / a_dy;

        let p_x = self.prize.0 as f64;
        let k_x = p_x / a_dx;

        let p_y = self.prize.1 as f64;
        let k_y = p_y / a_dy;

        let b = (k_y - k_x) / (r_y - r_x);
        let a = k_x - b * r_x;

        let mut result = if a.round() > 0.0 && b.round() > 0.0 {
            Some((a.round() as usize, b.round() as usize))
        } else {
            None
        };

        if result.is_some() {
            if a.round() as usize * self.button_a.0 + b.round() as usize * self.button_b.0
                != self.prize.0
            {
                result = None;
            }

            if a.round() as usize * self.button_a.1 + b.round() as usize * self.button_b.1
                != self.prize.1
            {
                result = None;
            }
        }

        result
    }
}

fn parse_machines(s: String) -> Vec<ClawMachine> {
    let mut machines = vec![];

    let mut current_machine = ClawMachine::default();
    for s in s.lines() {
        match s {
            s if s.starts_with("Button A: ") => {
                let (dx, dy) = s
                    .split_once(": ")
                    .expect("Incorrect format for A")
                    .1
                    .split_once(", ")
                    .expect("Incorrect format for A");

                current_machine.button_a = (
                    dx.split_once('+')
                        .expect("Bad A dx")
                        .1
                        .parse()
                        .expect("Bad A dx"),
                    dy.split_once('+')
                        .expect("Bad A dy")
                        .1
                        .parse()
                        .expect("Bad A dy"),
                );
            }
            s if s.starts_with("Button B: ") => {
                let (dx, dy) = s
                    .split_once(": ")
                    .expect("Incorrect format for B")
                    .1
                    .split_once(", ")
                    .expect("Incorrect format for B");

                current_machine.button_b = (
                    dx.split_once('+')
                        .expect("Bad B dx")
                        .1
                        .parse()
                        .expect("Bad B dx"),
                    dy.split_once('+')
                        .expect("Bad B dy")
                        .1
                        .parse()
                        .expect("Bad B dy"),
                );
            }
            s if s.starts_with("Prize: ") => {
                let (x, y) = s
                    .split_once(": ")
                    .expect("Incorrect format for Prize")
                    .1
                    .split_once(", ")
                    .expect("Incorrect format for Prize");

                current_machine.prize = (
                    x.split_once('=')
                        .expect("Bad Prize x")
                        .1
                        .parse()
                        .expect("Bad Prize x"),
                    y.split_once('=')
                        .expect("Bad Prize y")
                        .1
                        .parse()
                        .expect("Bad Prize y"),
                );

                machines.push(current_machine);
                current_machine = ClawMachine::default();
            }
            _ => {}
        }
    }

    machines
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let machines = parse_machines(input);

    let mut total_tokens = 0usize;
    for m in &machines {
        if let Some((a, b)) = m.calc_a_b() {
            total_tokens += a * 3 + b;
        }
    }
    println!("Part 1: {}", total_tokens);

    let modifier = 10000000000000;
    let mut total_tokens = 0usize;
    for mut m in machines {
        m.prize.0 += modifier;
        m.prize.1 += modifier;

        if let Some((a, b)) = m.calc_a_b() {
            total_tokens += a * 3 + b;
        }
    }
    println!("Part 2: {}", total_tokens);

    Ok(())
}
