use super::{Agent, Evaluation, Heuristic};
use crate::othello::*;

pub struct Minimax {
    heuristic: Box<dyn Heuristic>,
    expanded: usize,
    generated: usize,
}

impl Minimax {
    pub fn new(heuristic: Box<dyn Heuristic>) -> Box<dyn Agent> {
        Box::new(Minimax {
            heuristic: heuristic,
            expanded: 0,
            generated: 0,
        })
    }

    pub fn minimax(
        &mut self,
        game: &Othello,
        player: Disc,
        depth: usize,
    ) -> (Option<Position>, Evaluation) {
        self.expanded += 1;

        if depth == 0 {
            return (None, self.heuristic.eval(game, player));
        }

        // What possible moves can we make?
        let nodes = self.successors(game, player);

        // No possible moves from here, see what moves are in our opponent's future
        if nodes.len() == 0 {
            let (_, mut eval) = self.minimax(game, player.opponent(), depth - 1);
            eval = -eval;
            return (None, eval);
        }

        self.generated += nodes.len();

        // Now, maximize our benefit of each next move
        let mut max = (None, std::isize::MIN);

        for (m, board) in nodes.iter() {
            // Negate our opponent's next best move
            let (_, mut eval) = self.minimax(board, player.opponent(), depth - 1);
            eval = -eval;

            if eval > max.1 {
                max = (Some(*m), eval);
            }
        }

        max
    }

    fn successors(&self, game: &Othello, player: Disc) -> Vec<(Position, Othello)> {
        game.valid_moves(player)
            .iter()
            .map(|m| {
                let mut successor = game.clone();
                successor.place(*m, player);
                (*m, successor)
            })
            .collect()
    }
}

impl Agent for Minimax {
    fn solve(
        &mut self,
        game: &crate::othello::Othello,
        player: crate::othello::Disc,
        depth: usize,
    ) -> (Option<Position>, Evaluation) {
        self.expanded = 0;
        self.minimax(game, player, depth)
    }
    fn nodes_expanded(&self) -> usize {
        self.expanded
    }
    fn nodes_generated(&self) -> usize {
        self.generated
    }
}
