use super::{Evaluation, Heuristic, Node, Search};
use crate::othello::*;

pub struct AlphaBetaOrdering {
    heuristic: Box<dyn Heuristic>,
    expanded: usize,
    generated: usize,
}

impl AlphaBetaOrdering {
    pub fn new(heuristic: Box<dyn Heuristic>) -> Box<dyn Search> {
        Box::new(AlphaBetaOrdering {
            heuristic,
            expanded: 0,
            generated: 0,
        })
    }

    fn ab_order(
        &mut self,
        node: Node,
        game: &Othello,
        player: Disc,
        next_move: Disc,
        depth: usize,
        a: Evaluation,
        b: Evaluation,
        this_eval: Evaluation,
    ) -> (Option<Position>, Evaluation) {
        self.expanded += 1;

        if depth == 0 {
            return (None, this_eval);
        }

        // What possible moves can we make?
        let moves = self.successors(node, game, player, next_move);

        // No possible moves from here, see what moves are in our opponent's future
        if moves.len() == 0 {
            let (_, value) = self.ab_order(
                node.opposite(),
                game,
                player,
                next_move.opponent(),
                depth - 1,
                a,
                b,
                this_eval,
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
                let mut alpha = a;

                for (node_move, node_result, node_eval) in moves.iter() {
                    // Recursively call minimax to find the maximum value we can force
                    let (_, value) = self.ab_order(
                        Node::Min,
                        node_result,
                        player,
                        next_move.opponent(),
                        depth - 1,
                        alpha,
                        b,
                        *node_eval,
                    );

                    // Update local maximum
                    if value > best_value {
                        best_move = Some(*node_move);
                        best_value = value;
                    }

                    if best_value > alpha {
                        alpha = best_value;
                    }

                    if alpha >= b {
                        break;
                    }
                }

                (best_move, best_value)
            }
            Node::Min => {
                // Min node -> assume opponent will choose the worst possible move for us
                let mut worst_move = None;
                let mut worst_value = std::isize::MAX;
                let mut beta = b;

                for (node_move, node_game, node_eval) in moves.iter() {
                    // Recursively call minimax to find the minimum value they can force
                    let (_, value) = self.ab_order(
                        Node::Max,
                        node_game,
                        player,
                        next_move.opponent(),
                        depth - 1,
                        a,
                        beta,
                        *node_eval,
                    );

                    // Update local minimum
                    if value < worst_value {
                        worst_move = Some(*node_move);
                        worst_value = value;
                    }

                    // Update local beta value
                    if worst_value < b {
                        beta = worst_value;
                    }

                    if a >= beta {
                        break;
                    }
                }

                (worst_move, worst_value)
            }
        }
    }

    fn successors(
        &mut self,
        node: Node,
        game: &Othello,
        player: Disc,
        next_move: Disc,
    ) -> Vec<(Position, Othello, Evaluation)> {
        let mut moves: Vec<(Position, Othello, Evaluation)> = game
            .valid_moves(next_move)
            .iter()
            .map(|m| {
                let mut successor = game.clone();
                successor.place(*m, next_move);
                let eval = self.heuristic.eval(&successor, player);
                (*m, successor, eval)
            })
            .collect();

        match node {
            Node::Max => moves.sort_by(|(_, _, e1), (_, _, e2)| e2.cmp(&e1)),
            Node::Min => moves.sort_by(|(_, _, e1), (_, _, e2)| e1.cmp(&e2)),
        }

        moves
    }
}

impl Search for AlphaBetaOrdering {
    fn search(
        &mut self,
        game: &Othello,
        player: Disc,
        depth: usize,
    ) -> (Option<Position>, Evaluation) {
        self.expanded = 0;
        self.ab_order(
            Node::Max,
            game,
            player,
            player,
            depth,
            std::isize::MIN,
            std::isize::MAX,
            0,
        )
    }

    fn nodes_expanded(&self) -> usize {
        self.expanded
    }

    fn nodes_generated(&self) -> usize {
        self.generated
    }
}
