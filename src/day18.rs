use std::fmt::Debug;
use std::ops::Add;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18)]
pub fn input_generator(s: &str) -> Vec<SnailNumber> {
    s.lines()
        .filter_map(|line| SnailNumber::from_str(line).map(|(n, _)| n))
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(numbers: &Vec<SnailNumber>) -> u32 {
    let mut iter = numbers.iter();
    let mut first = iter.next().unwrap().clone();
    first.reduce();
    iter.fold(first.clone(), |a, b| {
        let res = &a + b;
        // println!("Res: {:?} + {:?} =\nRes: {:?}", a, b, res);
        res
    })
    .value()
}

#[aoc(day18, part2)]
pub fn solve_part2(numbers: &Vec<SnailNumber>) -> u32 {
	let mut res = 0;
	for l in numbers.iter() {
		for r in numbers.iter() {
			if l != r {
				res = res.max((l + r).value());
			}
		}
	}

	res
}

#[derive(Clone, PartialEq, Eq)]
pub enum SnailNumber {
    Branch(Box<SnailNumber>, Box<SnailNumber>),
    Value(u32),
}
use SnailNumber::*;

impl SnailNumber {
    fn from_str(remainder: &str) -> Option<(SnailNumber, &str)> {
        let first = remainder.chars().nth(0)?;

        if first.is_digit(10) {
            if let Some(first_non_digit) = remainder.find(|c: char| !c.is_digit(10)) {
                let (number, remainder) = remainder.split_at(first_non_digit);
                let number = number.parse().ok()?;
                Some((Value(number), remainder))
            } else {
                None
            }
        } else {
            if first != '[' {
                return None;
            }
            let (left, remainder) = SnailNumber::from_str(&remainder[1..])?;
            if remainder.chars().nth(0) != Some(',') {
                return None;
            }
            let (right, remaining) = SnailNumber::from_str(&remainder[1..])?;
            if remaining.chars().nth(0) != Some(']') {
                return None;
            }
            Some((Branch(Box::new(left), Box::new(right)), &remaining[1..]))
        }
    }

    fn value(&self) -> u32 {
        match self {
            Branch(l, r) => l.value() * 3 + r.value() * 2,
            Value(v) => *v,
        }
    }

    fn explode(&mut self) -> bool {
        self.do_explode(0).is_some()
    }

    fn do_explode(&mut self, depth: usize) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            Branch(left, right) if depth >= 4 && left.is_value() && right.is_value() => {
                let left = left.value();
                let right = right.value();
                *self = Value(0);
                Some((Some(left), Some(right)))
            }
            Branch(left, right) => {
                if let Some(todo) = left.do_explode(depth + 1) {
                    if let Some(value) = todo.1 {
                        right.add_leftmost(value);
                        Some((todo.0, None))
                    } else {
                        Some(todo)
                    }
                } else if let Some(todo) = right.do_explode(depth + 1) {
                    if let Some(value) = todo.0 {
                        left.add_rightmost(value);
                        Some((None, todo.1))
                    } else {
                        Some(todo)
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Branch(left, right) => left.split() || right.split(),
            Value(value) if *value >= 10 => {
                let left = *value / 2;
                let right = (*value + 1) / 2;
                *self = Branch(Box::new(Value(left)), Box::new(Value(right)));
                true
            }
            _ => false,
        }
    }

    fn add_leftmost(&mut self, value: u32) {
        match self {
            Branch(left, _right) => left.add_leftmost(value),
            Value(old_value) => *old_value += value,
        };
    }

    fn add_rightmost(&mut self, value: u32) {
        match self {
            Branch(_left, right) => right.add_rightmost(value),
            Value(old_value) => *old_value += value,
        };
    }

    fn reduce(&mut self) {
        // println!("Before reduce: {:?}", self);
        loop {
            if !self.explode() {
                if !self.split() {
                    break;
                } else {
                    // println!("After split: {:?}", self);
                }
            } else {
                // println!("After explode: {:?}", self);
            }
        }
    }

    fn is_value(&self) -> bool {
        if let Value(_) = self {
            true
        } else {
            false
        }
    }
}

impl Add for &SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Branch(Box::new(self.clone()), Box::new(rhs.clone()));
        res.reduce();
        res
    }
}

impl Debug for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Branch(arg0, arg1) => f.debug_list().entry(arg0).entry(arg1).finish(),
            Self::Value(arg0) => write!(f, "{}", arg0),
        }
    }
}
