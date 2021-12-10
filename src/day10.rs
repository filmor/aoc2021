use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParenKind {
    Round,
    Square,
    Brace,
    Angle,
}
use ParenKind::*;

impl ParenKind {
    fn value1(&self) -> usize {
        match self {
            Round => 3,
            Square => 57,
            Brace => 1197,
            Angle => 25137,
        }
    }

    fn value2(&self) -> usize {
        match self {
            Round => 1,
            Square => 2,
            Brace => 3,
            Angle => 4,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Paren {
    kind: ParenKind,
    open: bool,
}

impl Paren {
    fn from_char(c: char) -> Option<Self> {
        let (kind, open) = match c {
            '(' => (Round, true),
            ')' => (Round, false),
            '[' => (Square, true),
            ']' => (Square, false),
            '{' => (Brace, true),
            '}' => (Brace, false),
            '<' => (Angle, true),
            '>' => (Angle, false),
            _ => {
                return None;
            }
        };

        Some(Paren { kind, open })
    }
}

#[aoc_generator(day10)]
pub fn input_generator(s: &str) -> Vec<Vec<Paren>> {
    s.lines()
        .map(|l| l.chars().filter_map(Paren::from_char).collect())
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<Vec<Paren>>) -> usize {
    input
        .iter()
        .map(|line| {
            let mut stack = vec![];

            for p in line.iter() {
                let kind = p.kind;
                if !p.open {
                    if let Some(top) = stack.pop() {
                        if top != kind {
                            return kind.value1();
                        }
                    } else {
                        return kind.value1();
                    }
                } else {
                    stack.push(kind);
                }
            }

            0
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<Vec<Paren>>) -> usize {
    let mut scores: Vec<_> = input
        .iter()
        .filter_map(|line| {
            let mut stack = vec![];

            for p in line.iter() {
                let kind = p.kind;
                if !p.open {
                    if let Some(top) = stack.pop() {
                        if top != kind {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    stack.push(kind);
                }
            }

            Some(
            stack
                .iter()
                .rev()
                .fold(0, |acc, kind| acc * 5 + kind.value2())
            )
        })
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}
