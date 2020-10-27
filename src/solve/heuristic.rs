use super::{Evaluation, Heuristic};
use crate::othello::*;
use rand::{prelude::ThreadRng, thread_rng, Rng};

#[derive(Clone)]
pub struct HZero;

impl HZero {
    pub fn new() -> Box<HZero> {
        Box::new(HZero)
    }
}

impl Heuristic for HZero {
    fn eval(&mut self, _: &Othello, _: Disc) -> Evaluation {
        0
    }
}

#[derive(Clone)]
pub struct HRandom {
    rng: ThreadRng,
}

impl HRandom {
    pub fn new() -> Box<HRandom> {
        Box::new(HRandom { rng: thread_rng() })
    }
}

impl Heuristic for HRandom {
    fn eval(&mut self, _: &Othello, _: Disc) -> Evaluation {
        self.rng.gen::<isize>() % 10
    }
}

#[derive(Clone)]
pub struct HUnit;

impl HUnit {
    pub fn new() -> Box<HUnit> {
        Box::new(HUnit)
    }
}

impl Heuristic for HUnit {
    fn eval(&mut self, game: &Othello, player: Disc) -> Evaluation {
        let board = game.board();
        let opponent = player.opponent();
        let mut sum = 0;
        for row in 0..game.board().height() {
            for col in 0..game.board().width() {
                let pos = Position {
                    row: row as isize,
                    col: col as isize,
                };
                let disc = board.at(pos);
                if disc == player {
                    sum += 1;
                } else if disc == opponent {
                    sum -= 1;
                }
            }
        }
        sum
    }
}

const CELL_WEIGHTS: [[isize; 8]; 8] = [
    [120, -20, 20, 5, 5, 20, -20, 120],
    [-20, -40, -5, -5, -5, -5, -40, -20],
    [20, -5, 15, 3, 3, 15, -5, 20],
    [5, -5, 3, 3, 3, 3, -5, 5],
    [5, -5, 3, 3, 3, 3, -5, 5],
    [20, -5, 15, 3, 3, 15, -5, 20],
    [-20, -40, -5, -5, -5, -5, -40, -20],
    [120, -20, 20, 5, 5, 20, -20, 120],
];

#[derive(Clone)]
pub struct HWeighted;

impl HWeighted {
    pub fn new() -> Box<HWeighted> {
        Box::new(HWeighted)
    }
}

impl Heuristic for HWeighted {
    fn eval(&mut self, game: &Othello, player: Disc) -> Evaluation {
        let board = game.board();
        let opponent = player.opponent();
        let mut sum = 0;
        for row in 0..game.board().height() {
            for col in 0..game.board().width() {
                let pos = Position {
                    row: row as isize,
                    col: col as isize,
                };
                let disc = board.at(pos);
                if disc == player {
                    sum += CELL_WEIGHTS[row][col];
                } else if disc == opponent {
                    sum -= CELL_WEIGHTS[row][col];
                }
            }
        }
        sum
    }
}

#[derive(Clone)]
pub struct HMobility;

impl HMobility {
    pub fn new() -> Box<HMobility> {
        Box::new(HMobility)
    }
}

impl Heuristic for HMobility {
    fn eval(&mut self, game: &Othello, player: Disc) -> Evaluation {
        game.valid_moves(player).len() as isize
    }
}

#[derive(Clone)]
pub struct HWeightedMobility {
    weight: Box<HWeighted>,
    mobility: Box<HMobility>,
}

impl HWeightedMobility {
    pub fn new() -> Box<HWeightedMobility> {
        Box::new(HWeightedMobility {
            weight: HWeighted::new(),
            mobility: HMobility::new(),
        })
    }
}

impl Heuristic for HWeightedMobility {
    fn eval(&mut self, game: &Othello, player: Disc) -> Evaluation {
        self.weight.eval(game, player) + 5 * self.mobility.eval(game, player)
    }
}
