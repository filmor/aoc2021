use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(s: &str) -> Vec<u32> {
    s.lines()
        .filter_map(|n| u32::from_str_radix(n, 2).ok())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut count = 0;
    let mut bit_length = 0;
    let mut pos_counts = vec![0; 32];

    for n in input {
        let mut i = 0;

        let mut n = *n;
        while n != 0 {
            let bit = n & 1;
            n >>= 1;

            pos_counts[i] += bit;

            i += 1;
        }

        bit_length = bit_length.max(i);
        count += 1;
    }

    println!("{}", count);
    println!("{:?}", pos_counts);

    let gamma: u32 = pos_counts
        .into_iter()
        .take(bit_length)
        .map(|v| v > count / 2)
        .rev()
        .fold(0, |acc, b| (acc << 1) | b as u32);

	let epsilon = (u32::MAX >> 32 - bit_length) & !gamma;
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    unimplemented!()
}
