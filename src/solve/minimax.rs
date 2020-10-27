use super::{Evaluation, Heuristic, Node, Search};
use crate::othello::*;

pub struct Minimax {
    heuristic: Box<dyn Heuristic>,
    expanded: usize,
    generated: usize,
}

impl Minimax {
    pub fn new(heuristic: Box<dyn Heuristic>) -> Box<dyn Search> {
        Box::new(Minimax {
            heuristic,
            expanded: 0,
            generated: 0,
        })
    }

    fn minimax(
        &mut self,
        node: Node,
        game: &Othello,
        player: Disc,
        next_move: Disc,
        depth: usize,
    ) -> (Option<Position>, Evaluation) {
        self.expanded += 1;

        if depth == 0 {
            return (None, self.heuristic.eval(game, player));
        }

        // What possible moves can we make?
        let moves = self.successors(game, next_move);

        // No possible moves from here, see what moves are in our opponent's future
        if moves.len() == 0 {
            let (_, value) = self.minimax(
                node.opposite(),
                game,
                player,
                next_move.opponent(),
                depth - 1,
            );
            return (None, value);
        }

        self.generated += moves.len();

        // Now, maximize our benefit of each next move
        match node {
            Node::Max => {
                // Max node -> find the best possible move
                let mut best_move = None;
                let mut best_value = std::isize::MIN;

                for (node_move, node_game) in moves.iter() {
                    // Recursively call minimax to find the maximum value we can force
                    let (_, value) = self.minimax(
                        Node::Min,
                        node_game,
                        player,
                        next_move.opponent(),
                        depth - 1,
                    );

                    // Update local maximum
                    if value > best_value {
                        best_move = Some(*node_move);
                        best_value = value;
                    }
                }
                (best_move, best_value)
            }
            Node::Min => {
                // Min node -> assume opponent will choose the worst possible move for us
                let mut worst_move = None;
                let mut worst_value = std::isize::MAX;

                for (node_move, node_game) in moves.iter() {
                    // Recursively call minimax to find the minimum value they can force
                    let (_, value) = self.minimax(
                        Node::Max,
                        node_game,
                        player,
                        next_move.opponent(),
                        depth - 1,
                    );

                    // Update local minimum
                    if value < worst_value {
                        worst_move = Some(*node_move);
                        worst_value = value;
                    }
                }
                (worst_move, worst_value)
            }
        }
    }

    fn successors(&self, game: &Othello, next_move: Disc) -> Vec<(Position, Othello)> {
        game.valid_moves(next_move)
            .iter()
            .map(|m| {
                let mut successor = game.clone();
                successor.place(*m, next_move);
                (*m, successor)
            })
            .collect()
    }
}

impl Search for Minimax {
    fn search(
        &mut self,
        game: &Othello,
        player: Disc,
        depth: usize,
    ) -> (Option<Position>, Evaluation) {
        self.expanded = 0;
        self.minimax(Node::Max, game, player, player, depth)
    }

    fn nodes_expanded(&self) -> usize {
        self.expanded
    }

    fn nodes_generated(&self) -> usize {
        self.generated
    }
}
