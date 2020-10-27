mod alphabeta;
mod ab_order;
mod ab_order_unit;
mod heuristic;
mod minimax;

pub use alphabeta::*;
pub use ab_order::*;
pub use ab_order_unit::*;
pub use heuristic::*;
pub use minimax::*;

use crate::othello::{Disc, Othello, Position};

pub type Evaluation = isize;

pub trait Heuristic {
    fn eval(&mut self, game: &Othello, player: Disc) -> Evaluation;
}

pub trait Search {
    fn search(
        &mut self,
        game: &Othello,
        player: Disc,
        depth: usize,
    ) -> (Option<Position>, Evaluation);

    fn nodes_expanded(&self) -> usize;
    fn nodes_generated(&self) -> usize;
}

#[derive(Copy, Clone)]
enum Node {
    Max,
    Min,
}

impl Node {
    fn opposite(&self) -> Node {
        match self {
            Node::Min => Node::Max,
            Node::Max => Node::Min,
        }
    }
}
