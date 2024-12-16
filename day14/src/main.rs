use core::f64;
use std::{error::Error, fs};

#[derive(Debug, Clone, Copy)]
struct Bot {
    x: usize,
    y: usize,
    vx: isize,
    vy: isize,
}

impl Bot {
    fn move_it(&mut self, w: usize, h: usize) {
        let mut next_x = self.x as isize + self.vx;
        if next_x < 0 {
            next_x += w as isize;
        } else if next_x >= w as isize {
            next_x -= w as isize;
        }

        let mut next_y = self.y as isize + self.vy;
        if next_y < 0 {
            next_y += h as isize;
        } else if next_y >= h as isize {
            next_y -= h as isize;
        }

        self.x = next_x as usize;
        self.y = next_y as usize;
    }
}

fn print_map(w: usize, h: usize, bots: &[Bot]) {
    let mx = (w - 1) / 2;
    let my = (h - 1) / 2;

    let mut xy = vec![vec![0usize; w]; h];

    for j in 0..h {
        for i in 0..w {
            xy[j][i] = bots.iter().filter(|b| b.x == i && b.y == j).count();
        }
    }

    let mut quads = [0usize; 4];

    for j in 0..h {
        for i in 0..w {
            let q: usize = if j < my { 0 } else { 2 } + if i < mx { 0 } else { 1 };

            if j != my && i != mx {
                quads[q] += xy[j][i];
            }

            let v = match xy[j][i] {
                _ if j == my => " ".to_owned(),
                _ if i == mx => " ".to_owned(),
                0 => ".".to_string(),
                n => n.to_string(),
            };

            print!("{}", v)
        }
        println!();
    }

    println!();
    println!("Quads: {:?}", quads);
    println!("Safety: {}", quads.iter().product::<usize>());
}

fn get_variance(w: usize, h: usize, bots: &[Bot]) -> (isize, isize) {
    let mut xy = vec![vec![0usize; w]; h];

    for j in 0..h {
        for i in 0..w {
            xy[j][i] = bots.iter().filter(|b| b.x == i && b.y == j).count();
        }
    }

    let kx = bots[0].x;
    let ky = bots[0].y;

    let mut ex: isize = 0;
    let mut ey: isize = 0;
    let mut ex2: isize = 0;
    let mut ey2: isize = 0;

    for b in bots {
        ex += b.x as isize - kx as isize;
        ey += b.y as isize - ky as isize;
        ex2 += isize::pow(b.x as isize - kx as isize, 2);
        ey2 += isize::pow(b.y as isize - ky as isize, 2);
    }

    let n = bots.len() as isize;
    let vx = (ex2 - isize::pow(ex, 2) / n) / (n - 1);
    let vy = (ey2 - isize::pow(ey, 2) / n) / (n - 1);

    (vx, vy)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut bots = input
        .lines()
        .filter_map(|s| s.split_once("p=").and_then(|s| s.1.split_once(" v=")))
        .filter_map(|(pos, speed)| {
            let (x, y) = pos.split_once(',')?;
            let (vx, vy) = speed.split_once(',')?;

            Some(Bot {
                x: x.parse().ok()?,
                y: y.parse().ok()?,
                vx: vx.parse().ok()?,
                vy: vy.parse().ok()?,
            })
        })
        .collect::<Vec<_>>();

    let w = 101;
    let h = 103;

    let mut min_variance = f64::MAX;
    let mut found_t = None;
    let mut found_bots = None;

    for t in 0..10_000 {
        bots.iter_mut().for_each(|b| b.move_it(w, h));

        let (vx, vy) = get_variance(w, h, &bots);

        let sqv = f64::sqrt(isize::pow(vx, 2) as f64 + isize::pow(vy, 2) as f64);

        if sqv < min_variance {
            min_variance = sqv;
            found_t = Some(t);
            found_bots = Some(bots.clone());
        }

        if t % 50 == 0 {
            println!("Elapsed time: {}, Min variance: {}", t + 1, min_variance);
        }

        if t == 99 {
            println!("100 seconds");
            print_map(w, h, &bots);
        }
    }

    if let Some(t) = found_t {
        if let Some(bots) = found_bots {
            println!("XMAS TREE FOUND AT: {}", t + 1);
            print_map(w, h, &bots);
        }
    }

    Ok(())
}
