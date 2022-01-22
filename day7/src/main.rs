use std::{collections::HashMap, fs::read_to_string};

fn main() {
    // min and max positions of crabs
    let mut min_pos = isize::MAX;
    let mut max_pos = 0;

    let positions: Vec<isize> = read_to_string("./input.txt")
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .map(|pos| {
            if pos < min_pos {
                min_pos = pos;
            }

            if pos > max_pos {
                max_pos = pos;
            }

            pos
        })
        .collect();

    let mut pos_move_counter: HashMap<isize, isize> = HashMap::new();

    positions.iter().for_each(|&pos| {
        (min_pos..=max_pos).for_each(|p| {
            let count = pos_move_counter.entry(p).or_insert(0);
            let diff: isize = (pos - p) as isize;
            *count += diff.abs();
        })
    });

    let min_fuel_count = pos_move_counter
        .iter()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    dbg!(min_fuel_count);
}
