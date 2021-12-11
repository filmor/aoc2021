use std::{collections::HashSet, fmt::Display};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn input_generator(s: &str) -> Field {
    Field::from_str(s)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Field) -> usize {
    let mut f = input.clone();
	let mut res = 0;
	for _ in 0..100 {
		res += f.step();
	}

    res
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Field) -> usize {
    let mut f = input.clone();
	for i in 0.. {
		if f.step() == f.data.len() {
			return i + 1;
		}
	}

	unreachable!()
}

#[derive(Clone)]
pub struct Field {
    data: Vec<i8>,
    width: usize,
    height: usize,
}

impl Field {
    fn from_str(s: &str) -> Field {
        let mut data = vec![];

        let mut width = 0;
        let mut height = 0;
        for l in s.lines() {
            width = width.max(l.len());
            data.extend(l.chars().filter_map(|c| c.to_digit(10).map(|i| i as i8)));
            height += 1;
        }

        Field {
            data,
            width,
            height,
        }
    }

    fn get_index(&self, p: (isize, isize)) -> Option<usize> {
        if p.0 < 0 || p.0 >= self.width as isize || p.1 < 0 || p.1 >= self.height as isize {
            None
        } else {
            Some((p.0 + p.1 * (self.width as isize)) as usize)
        }
    }

    fn get(&self, idx: (isize, isize)) -> Option<i8> {
        self.get_index(idx).map(|idx| self.data[idx])
    }

    fn step(&mut self) -> usize {
        let mut flashes = HashSet::new();

        for i in 0..self.width as isize {
            for j in 0..self.height as isize {
                self.maybe_flash(&mut flashes, (i, j))
            }
        }

		for p in flashes.iter() {
			if let Some(idx) = self.get_index(*p) {
				self.data[idx] = 0;
			}
		}

        flashes.len()
    }

    fn maybe_flash(&mut self, flashes: &mut HashSet<(isize, isize)>, p: (isize, isize)) {
        if flashes.contains(&p) {
            return;
        }

		if let Some(idx) = self.get_index(p) {
			self.data[idx] += 1;
			if self.data[idx] > 9 {
				flashes.insert(p);
				for d_x in -1..=1 {
					for d_y in -1..=1 {
						let p1 = (p.0 + d_x, p.1 + d_y);
						self.maybe_flash(flashes, p1);
					}
				}
			}
		}
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height as isize {
            for j in 0..self.width as isize {
                write!(f, "{} ", self.get((i, j)).unwrap())?
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
