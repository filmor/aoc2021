use std::{collections::HashSet, fmt::Display};

use aoc_runner_derive::{aoc, aoc_generator};

type Number = u8;

#[aoc_generator(day4)]
pub fn input_generator(s: &str) -> Game {
    let mut it = s.split_ascii_whitespace();

    let draw = it
        .next()
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut boards = vec![];

    let mut it = it.filter_map(|n| n.parse().ok());

    loop {
        if let Some(board) = Board::try_from_iter(&mut it) {
            // println!("{}", board);
            boards.push(board);
        } else {
            break;
        }
    }

    Game { draw, boards }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Game) -> u32 {
    let mut game_state = input.start();

    loop {
        match game_state.draw() {
            DrawResult::Bingo(winners) => break winners.first().unwrap().score,
            DrawResult::NoBingo => continue,
            DrawResult::End => panic!("No winner found"),
        }
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Game) -> u32 {
    let mut game_state = input.start();
    let count = input.boards.len();
    let mut all_winners: HashSet<_> = (0..count).collect();

    loop {
        match game_state.draw() {
            DrawResult::Bingo(winners) => {
                for winner in winners.iter() {
                    all_winners.remove(&winner.board);
                    // println!("{:?} {}", &all_winners, winner.score);
                    if all_winners.is_empty() {
                        return winner.score;
                    }
                }
            }
            DrawResult::End => panic!("No winner found"),
            _ => {}
        }
    }
}

pub struct Game {
    draw: Vec<Number>,
    boards: Vec<Board>,
}

impl Game {
    fn start<'a>(&'a self) -> GameState<'a> {
        GameState {
            game: self,
            round: 0,
            states: self.boards.iter().map(BoardState::new).collect(),
        }
    }
}

struct Board([Number; 25]);

impl Board {
    fn try_from_iter(it: &mut impl Iterator<Item = Number>) -> Option<Board> {
        let mut res = Board([0; 25]);
        for i in 0..25 {
            if let Some(val) = it.next() {
                res.0[i] = val;
            } else {
                return None;
            }
        }

        Some(res)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.chunks(5) {
            write!(
                f,
                "{:3} {:3} {:3} {:3} {:3}\n",
                row[0], row[1], row[2], row[3], row[4]
            )?;
        }

        write!(f, "\n")
    }
}

struct GameState<'a> {
    game: &'a Game,
    round: usize,
    states: Vec<BoardState<'a>>,
}

impl GameState<'_> {
    fn draw(&mut self) -> DrawResult {
        if let Some(draw) = self.game.draw.get(self.round) {
            self.round += 1;
            let mut winners = vec![];
            for b in 0..self.states.len() {
                if let Some(bingo) = self.states[b].check_draw(*draw) {
                    winners.push(Winner {
                        board: b,
                        bingo,
                        score: self.states[b].sum_unmarked() * *draw as u32,
                    });
                }
            }
            if winners.len() > 0 {
                DrawResult::Bingo(winners)
            } else {
                DrawResult::NoBingo
            }
        } else {
            DrawResult::End
        }
    }
}

#[derive(Debug)]
enum DrawResult {
    Bingo(Vec<Winner>),
    NoBingo,
    End,
}

#[derive(Debug, Clone)]
struct Winner {
    board: usize,
    bingo: Bingo,
    score: u32,
}

#[derive(Clone, Copy)]
struct BoardState<'a> {
    state: u32,
    board: &'a Board,
}

impl BoardState<'_> {
    fn new<'a>(board: &'a Board) -> BoardState<'a> {
        BoardState { state: 0, board }
    }

    fn sum_unmarked(&self) -> u32 {
        let mut res = 0;
        for i in 0..25 {
            if 1 << i & self.state == 0 {
                res += self.board.0[i] as u32;
            }
        }
        res
    }

    fn check_draw(&mut self, number: Number) -> Option<Bingo> {
        self.board
            .0
            .iter()
            .position(|&val| val == number)
            .and_then(|pos| self.set(pos))
    }

    fn set(&mut self, n: usize) -> Option<Bingo> {
        self.state |= 1 << n;
        let col = n % 5;
        let row = n / 5;
        assert!(self.get(col, row));

        // println!("{}", self);
        self.check_row(row).or(self.check_column(col))
        /* if col == row {
            self.check_diag()?;
        }
        if 5 - col == row {
            self.check_antidiag()?;
        } */
    }

    // fn check_diag(&self) -> Option<Bingo> {
    //     const MASK: u32 = 1 << (0 * 5 + 0)
    //         | 1 << (1 * 5 + 1)
    //         | 1 << (2 * 5 + 2)
    //         | 1 << (3 * 5 + 3)
    //         | 1 << (4 * 5 + 4);
    //     (MASK & self.state == MASK).then(|| Bingo::Diag)
    // }

    // fn check_antidiag(&self) -> Option<Bingo> {
    //     const MASK: u32 = 1 << (0 * 5 + 4)
    //         | 1 << (1 * 5 + 3)
    //         | 1 << (2 * 5 + 2)
    //         | 1 << (3 * 5 + 1)
    //         | 1 << (4 * 5 + 0);
    //     (MASK & self.state == MASK).then(|| Bingo::AntiDiag)
    // }

    const fn check_column(&self, col: usize) -> Option<Bingo> {
        const MASK: u32 = 1 << 0 * 5 | 1 << 1 * 5 | 1 << 2 * 5 | 1 << 3 * 5 | 1 << 4 * 5;
        let mask = MASK << col;
        // println!("Col: {}\nVal:  {:025b}\nMask: {:025b}\nComb: {:025b}\n", col, self.0, mask, self.0 & mask);
        // println!("Comb == Mask: {}", self.0 & mask == mask);
        if mask & self.state == mask {
            Some(Bingo::Column(col))
        } else {
            None
        }
    }

    const fn check_row(&self, row: usize) -> Option<Bingo> {
        const MASK: u32 = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4;
        let mask = MASK << row * 5;
        if mask & self.state == mask {
            Some(Bingo::Row(row))
        } else {
            None
        }
    }

    const fn get(&self, col: usize, row: usize) -> bool {
        1 << (row * 5 + col) & self.state != 0
    }
}

impl Display for BoardState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                write!(f, "{}", if self.get(row, col) { "  X" } else { "  ." })?;
            }

            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

#[derive(Debug, Clone)]
enum Bingo {
    // Diag,
    // AntiDiag,
    Row(usize),
    Column(usize),
}
