use super::{Board, Disc, Position};

const DIRECTIONS: &[(isize, isize)] = &[
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

#[derive(Clone)]
pub struct Move {
    index: usize,
    tile: Disc,
}

#[derive(Clone)]
pub struct Othello {
    board: Board,
}

impl Othello {
    pub fn new(board: Board) -> Self {
        Othello { board }
    }

    pub fn valid_moves(&self, player: Disc) -> Vec<Position> {
        let mut moves = Vec::new();
        for row in 0..self.board.height() {
            for col in 0..self.board.width() {
                let pos = Position::new(row as isize, col as isize);
                if self.is_valid_move(pos, player) {
                    moves.push(pos);
                }
            }
        }
        moves
    }

    pub fn is_valid_move(&self, pos: Position, player: Disc) -> bool {
        // If this isn't empty, of course we can't move here
        if self.board.at(pos) != Disc::Empty {
            return false;
        }

        DIRECTIONS
            .iter()
            .any(|direction| self.is_valid_move_dir(pos, player, *direction))
    }

    fn is_valid_move_dir(&self, pos: Position, player: Disc, (dr, dc): (isize, isize)) -> bool {
        let mut step = 1;

        // The opposite color must be between this coordinate and another of the same disc
        let between = player.opponent();
        let mut between_satisfied = false;

        loop {
            let new_row = pos.row + step * dr;
            let new_col = pos.col + step * dc;

            if new_row < 0
                || new_col < 0
                || new_row >= self.board.height() as isize
                || new_col >= self.board.width() as isize
            {
                return false;
            }

            // println!("new_row = {}, new_col = {}", new_row, new_col);
            let new_pos = Position::new(new_row, new_col);

            let disc = self.board.at(new_pos);

            if disc == Disc::Empty {
                // If we get to an empty disc, we can't move here
                // It's either the edge or an empty tile
                return false;
            } else if disc == between {
                // The between part is satisfied, so let's see if we can find one of the same disc
                between_satisfied = true;
            } else {
                if between_satisfied {
                    return true;
                } else {
                    return false;
                }
            }

            step += 1;
        }
    }

    pub fn place(&mut self, pos: Position, player: Disc) {
        debug_assert!(self.is_valid_move(pos, player));
        self.board.set(pos, player);
        for dir in DIRECTIONS.iter() {
            if self.is_valid_move_dir(pos, player, *dir) {
                self.place_dir(pos, player, *dir);
            }
        }
    }

    fn place_dir(&mut self, pos: Position, player: Disc, (dr, dc): (isize, isize)) {
        let mut step = 1;

        // The opposite color must be between this coordinate and another of the same disc
        let between = player.opponent();

        loop {
            let new_row = pos.row + step * dr;
            let new_col = pos.col + step * dc;

            if new_row < 0
                || new_col < 0
                || new_row >= self.board.height() as isize
                || new_col >= self.board.width() as isize
            {
                break;
            }

            // println!("new_row = {}, new_col = {}", new_row, new_col);
            let new_pos = Position::new(new_row, new_col);
            let disc = self.board.at(new_pos);

            if disc == between {
                self.board.set(new_pos, player);
            } else {
                break;
            }

            step += 1;
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn any_valid_moves(&self) -> bool {
        for row in 0..self.board.height() {
            for col in 0..self.board.width() {
                let pos = Position::new(row as isize, col as isize);

                if self.is_valid_move(pos, Disc::Black) || self.is_valid_move(pos, Disc::White) {
                    return true;
                }
            }
        }
        false
    }

    pub fn winner(&self) -> Option<Disc> {
        let mut white = 0;
        let mut black = 0;
        for row in 0..self.board.height() {
            for col in 0..self.board.width() {
                let pos = Position::new(row as isize, col as isize);
                let disc = self.board.at(pos);
                if disc == Disc::White {
                    white += 1;
                } else if disc == Disc::Black {
                    black += 1;
                }
            }
        }

        if white > black {
            Some(Disc::White)
        } else if black > white {
            Some(Disc::Black)
        } else {
            None
        }
    }
}
