use std::{error::Error, fs};

fn disk_map(input: &str) -> Vec<Option<usize>> {
    let mut expanded: Vec<Option<usize>> = vec![];

    for i in 0..input.len() - 1 {
        let is_file = i % 2 == 0;
        let file_id = if is_file { Some(i / 2) } else { None };

        if let Some(len) = input
            .chars()
            .nth(i)
            .expect("Premature input end")
            .to_digit(10)
        {
            for _ in 0..len {
                expanded.push(file_id);
            }
        }
    }

    expanded
}

fn checksum(disk_map: &[Option<usize>]) -> usize {
    disk_map.iter().enumerate().fold(0usize, |acc, (i, c)| {
        acc + i * match c {
            None => 0,
            Some(id) => *id,
        }
    })
}

fn defrag_part1(disk_map: &mut [Option<usize>]) {
    let mut l = 0usize;
    let mut r = disk_map.len() - 1;

    while l < r {
        while disk_map[l].is_some() && l < r {
            l += 1;
        }

        while disk_map[r].is_none() && l < r {
            r -= 1;
        }

        if l != r {
            disk_map[l] = disk_map[r];
            disk_map[r] = None;
        }
    }
}

fn defrag_part2(disk_map: &mut [Option<usize>]) {
    let mut free = vec![];
    let mut files = vec![];

    let mut start = 0;
    for i in 1..disk_map.len() {
        let mut end = None;

        if disk_map[i] == disk_map[start] && i < disk_map.len() - 1 {
            continue;
        }

        if disk_map[i] != disk_map[start] {
            end = Some(i - 1);
        } else if i == disk_map.len() - 1 {
            end = Some(i);
        }

        if let Some(end) = end {
            match disk_map[end] {
                Some(id) => files.push((id, (start, end))),
                None => free.push((start, end)),
            }
        }

        start = i;
    }

    let mut moved = vec![];

    while let Some(file) = files.pop() {
        let (file_id, file) = file;
        let file_size = file.1 - file.0;

        let mut matching_slot = None;
        for (i, slot) in free.iter().enumerate() {
            let slot_size = slot.1 - slot.0;
            if slot_size >= file_size && slot.1 < file.0 {
                matching_slot = Some(i);
                break;
            }
        }

        if let Some(slot_idx) = matching_slot {
            let mut need_remove = false;
            if let Some(slot) = free.get_mut(slot_idx) {
                let slot_size = slot.1 - slot.0;

                if slot_size == file_size {
                    moved.push((file_id, *slot));
                    need_remove = true;
                }

                if slot_size > file_size {
                    moved.push((file_id, (slot.0, slot.0 + file_size)));
                    slot.0 += file_size + 1;
                }
            }

            if need_remove {
                free.remove(slot_idx);
            }
        } else {
            moved.push((file_id, file));
        }
    }

    let len = disk_map.len();
    for i in 0..len {
        disk_map[i] = None;
    }
    for (id, (start, end)) in moved {
        for i in start..=end {
            disk_map[i] = Some(id);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let disk_map = disk_map(&input);
    println!("Checksum: {}", checksum(&disk_map));

    let mut part1_map = disk_map.clone();
    defrag_part1(&mut part1_map);
    println!("Part 1: {}", checksum(&part1_map));

    let mut part2_map = disk_map.clone();
    defrag_part2(&mut part2_map);
    println!("Part 2: {}", checksum(&part2_map));

    Ok(())
}
