use super::{Evaluation, Heuristic, Node, Search};
use crate::othello::*;

const BOARD_STEP: usize = 2;

pub struct AlphaBetaOrderingUnit {
    heuristic: Box<dyn Heuristic>,
    expanded: usize,
    generated: usize,
}

impl AlphaBetaOrderingUnit {
    pub fn new(heuristic: Box<dyn Heuristic>) -> Box<dyn Search> {
        Box::new(AlphaBetaOrderingUnit {
            heuristic,
            expanded: 0,
            generated: 0,
        })
    }

    fn alphabeta(
        &mut self,
        node: Node,
        game: &Othello,
        player: Disc,
        next_move: Disc,
        depth: usize,
        a: Evaluation,
        b: Evaluation,
    ) -> (Option<Position>, Evaluation) {
        self.expanded += 1;

        if depth == 0 {
            return (None, self.heuristic.eval(game, player));
        }

        // What possible moves can we make?
        let moves = self.successors(game, next_move, player, node);

        // No possible moves from here, see what moves are in our opponent's future
        if moves.len() == 0 {
            let (_, value) = self.alphabeta(
                node.opposite(),
                game,
                player,
                next_move.opponent(),
                depth - 1,
                a,
                b,
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

                for (node_move, node_game) in moves.iter() {
                    // Recursively call minimax to find the maximum value we can force
                    let (_, value) = self.alphabeta(
                        Node::Min,
                        node_game,
                        player,
                        next_move.opponent(),
                        depth - 1,
                        alpha,
                        b,
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

                for (node_move, node_game) in moves.iter() {
                    // Recursively call minimax to find the minimum value they can force
                    let (_, value) = self.alphabeta(
                        Node::Max,
                        node_game,
                        player,
                        next_move.opponent(),
                        depth - 1,
                        a,
                        beta,
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

    fn successors(&self, game: &Othello, next_move: Disc, player: Disc, node: Node) -> Vec<(Position, Othello)> {
        let mut moves: Vec<(Position, Othello, Evaluation)> = game.valid_moves(next_move)
            .iter()
            .map(|m| {
                let mut successor = game.clone();
                successor.place(*m, next_move);
                let eval = self.ordering_function(game, player);
                (*m, successor, eval)
            })
            .collect();

        match node {
            Node::Max => moves.sort_by(|(_, _, e1), (_, _, e2)| e2.cmp(&e1)),
            Node::Min => moves.sort_by(|(_, _, e1), (_, _, e2)| e1.cmp(&e2)),
        }
    
        moves.into_iter().map(|(p, g, _)| (p, g)).collect()
    }

    fn ordering_function(&self, game: &Othello, player: Disc) -> Evaluation {
        let board = game.board();
        let opponent = player.opponent();
        let mut eval = 0;

        for row in (0..board.height()).step_by(BOARD_STEP) {
            for col in (0..board.width()).step_by(BOARD_STEP) {
                let disc = board.at(Position::new(row as isize, col as isize));
                if disc == player {
                    eval += 1;
                } else if disc == opponent {
                    eval -= 1;
                }   
            }
        }

        eval
    }
}

impl Search for AlphaBetaOrderingUnit {
    fn search(
        &mut self,
        game: &Othello,
        player: Disc,
        depth: usize,
    ) -> (Option<Position>, Evaluation) {
        self.expanded = 0;
        self.alphabeta(
            Node::Max,
            game,
            player,
            player,
            depth,
            std::isize::MIN,
            std::isize::MAX,
        )
    }

    fn nodes_expanded(&self) -> usize {
        self.expanded
    }

    fn nodes_generated(&self) -> usize {
        self.generated
    }
}
