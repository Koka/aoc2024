use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

fn get_antinodes(
    antennas: &[(usize, usize)],
    w: usize,
    h: usize,
    resonant: bool,
) -> HashSet<(isize, isize)> {
    let mut antinodes = HashSet::new();

    let min_distance: isize = if resonant { 1 } else { 2 };
    let max_distance: isize = if resonant { max(w, h) as isize } else { 2 };

    for i in 0..antennas.len() - 1 {
        for j in i + 1..antennas.len() {
            let l = antennas[i];
            let r = antennas[j];

            let dh: isize = r.0 as isize - l.0 as isize;
            let dw: isize = r.1 as isize - l.1 as isize;

            for step in min_distance..=max_distance {
                let antinode_a = (r.0 as isize - step * dh, r.1 as isize - step * dw);
                if antinode_a.0 >= 0
                    && antinode_a.0 < h.try_into().unwrap()
                    && antinode_a.1 >= 0
                    && antinode_a.1 < w.try_into().unwrap()
                {
                    antinodes.insert(antinode_a);
                }

                let antinode_b = (l.0 as isize + step * dh, l.1 as isize + step * dw);
                if antinode_b.0 >= 0
                    && antinode_b.0 < h.try_into().unwrap()
                    && antinode_b.1 >= 0
                    && antinode_b.1 < w.try_into().unwrap()
                {
                    antinodes.insert(antinode_b);
                }
            }
        }
    }

    antinodes
}

fn part1(antennas: &HashMap<char, Vec<(usize, usize)>>, w: usize, h: usize) {
    let mut all_antinodes = HashSet::new();

    for freq in antennas.keys() {
        let antennas = antennas.get(freq).unwrap();

        let antinodes = get_antinodes(antennas, w, h, false);

        println!("{}: {:?} = {} antinodes", freq, antennas, antinodes.len());

        for n in antinodes {
            all_antinodes.insert(n);
        }
    }

    println!("Part 1: {}", all_antinodes.len());
}

fn part2(antennas: &HashMap<char, Vec<(usize, usize)>>, w: usize, h: usize) {
    let mut all_antinodes = HashSet::new();

    for freq in antennas.keys() {
        let antennas = antennas.get(freq).unwrap();

        let antinodes = get_antinodes(antennas, w, h, true);

        println!("{}: {:?} = {} antinodes", freq, antennas, antinodes.len());

        for n in antinodes {
            all_antinodes.insert(n);
        }
    }

    println!("Part 2: {}", all_antinodes.len());
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(i, s)| {
            s.chars().enumerate().filter_map(move |(j, c)| match c {
                '.' => None,
                c => Some((c, i, j)),
            })
        })
        .fold(HashMap::new(), |mut acc, (freq, i, j)| {
            acc.entry(freq)
                .and_modify(|v: &mut Vec<(usize, usize)>| {
                    v.push((i, j));
                })
                .or_insert_with(|| vec![(i, j)]);

            acc
        });

    println!("{}x{}x{}", w, h, antennas.len());

    part1(&antennas, w, h);
    part2(&antennas, w, h);

    Ok(())
}
