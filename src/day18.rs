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
    numbers
        .iter()
        .fold(SnailNumber::default(), |a, b| a + b.clone())
        .value()
}

#[derive(Clone, Debug)]
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
            Branch(left, right) if depth > 4 && left.is_value() && right.is_value() => {
                Some((Some(left.value()), Some(right.value())))
            }
            Branch(left, right) => {
                if let Some(todo) = left.do_explode(depth + 1) {
                    if let Some(value) = todo.0 {
                        right.set_leftmost(value);
                        Some((None, todo.1))
                    } else {
                        Some(todo)
                    }
                } else if let Some(todo) = right.do_explode(depth + 1) {
                    if let Some(value) = todo.1 {
                        left.set_rightmost(value);
                        Some((todo.1, None))
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
                let new_val = *value / 2;
                *self = Branch(Box::new(Value(new_val)), Box::new(Value(new_val)));
                true
            }
            _ => false,
        }
    }

    fn set_leftmost(&mut self, value: u32) {
        match self {
            Branch(left, _right) => left.set_leftmost(value),
            Value(old_value) => *old_value = value,
        };
    }

    fn set_rightmost(&mut self, value: u32) {
        match self {
            Branch(_left, right) => right.set_rightmost(value),
            Value(old_value) => *old_value = value,
        };
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
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

impl Add for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Branch(Box::new(self), Box::new(rhs));
        res.reduce();
        res
    }
}

impl Default for SnailNumber {
    fn default() -> Self {
        Value(0)
    }
}
