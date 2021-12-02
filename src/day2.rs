use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    value: i32,
}

type Position = (i32, i32);

impl Command {
    fn apply1(&self, pos: &mut Position) {
        match self.direction {
            Direction::Up => pos.1 -= self.value,
            Direction::Down => pos.1 += self.value,
            Direction::Forward => pos.0 += self.value,
        }
    }

    fn apply2(&self, state: &mut State) {
        match self.direction {
            Direction::Up => state.aim -= self.value,
            Direction::Down => state.aim += self.value,
            Direction::Forward => {
                state.position.0 += self.value;
                state.position.1 += self.value * state.aim;
            }
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = s.split_once(" ").ok_or(())?;

        let direction = match direction {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => {
                return Err(());
            }
        };

        let value = value.parse().map_err(|_| ())?;

        Ok(Command { direction, value })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(s: &str) -> Vec<Command> {
    s.lines().filter_map(|n| n.parse().ok()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Command]) -> u32 {
    let mut pos = Position::default();
    for cmd in input {
        // let old_pos = pos;
        cmd.apply1(&mut pos);
        // println!("{:?}: {:?} => {:?}", cmd, old_pos, pos);
    }

    (pos.0 * pos.1) as u32
}

#[derive(Debug, Default)]
struct State {
    aim: i32,
    position: Position,
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> u32 {
    let mut state = State::default();
    for cmd in input {
        // let old_pos = pos;
        cmd.apply2(&mut state);
        // println!("{:?}: {:?} => {:?}", cmd, old_pos, pos);
    }

    (state.position.0 * state.position.1) as u32
}
