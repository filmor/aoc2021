use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(s: &str) -> Vec<u8> {
    s.split(",").filter_map(|n| n.parse().ok()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<u8>) -> usize {
    let mut fishes = input.clone();

    for _ in 0..80 {
        let mut to_add = 0;

        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                to_add += 1
            } else {
                *fish -= 1;
            }
        }

        for _ in 0..to_add {
            fishes.push(8);
        }
    }

    fishes.len()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<u8>) -> usize {
    let mut fishes = [0usize; 9];
    for &i in input.iter() {
        fishes[i as usize] += 1;
    }

    for shift in 1..=256 {
        fishes[(shift + 6) % 9] += fishes[(shift - 1) % 9];
    }

    fishes.iter().sum()
}
