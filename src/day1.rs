use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(s: &str) -> Vec<u32> {
    s.split_ascii_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(l, r)| l < r)
        .count() as u32
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    // Sliding windows are a red herring. If we compare (a + b + c) < (b + c + d), then that's
    // equivalent to a < d
    input
        .iter()
        .zip(input.iter().skip(3))
        .filter(|(a, d)| a < d)
        .count() as u32
}
