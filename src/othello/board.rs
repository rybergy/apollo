use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Disc {
    Black,
    White,
    Empty,
}

impl Disc {
    pub fn opponent(&self) -> Disc {
        match self {
            Disc::Black => Disc::White,
            Disc::White => Disc::Black,
            Disc::Empty => panic!("attempting to take opposite of empty disc!"),
        }
    }
}

impl Display for Disc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Disc::Empty => "   ",
            Disc::Black => "███",
            Disc::White => "░░░",
            // Tile::Empty => "   ",
            // Tile::Black => " ● ",
            // Tile::White => " ○ ",
        };
        write!(f, "{}", c)
    }
}
#[derive(Hash, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub row: isize,
    pub col: isize,
}

impl Position {
    pub fn new(row: isize, col: isize) -> Self {
        Position { row, col }
    }
}

/// The Othello board.
///
/// It is represented by a 1-dimensional vector,
/// where there are empty spaces at every edge
/// of the board.
#[derive(Clone)]
pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Disc>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            tiles: (0..((width + 2) * (height + 2)))
                .map(|_| Disc::Empty)
                .collect(),
        }
    }

    pub fn black(&mut self, pos: Position) -> &mut Self {
        self.set(pos, Disc::Black);
        self
    }

    pub fn white(&mut self, pos: Position) -> &mut Self {
        self.set(pos, Disc::White);
        self
    }

    pub fn set(&mut self, pos: Position, tile: Disc) {
        let index = self.pos(pos);
        let change = self.tiles.get_mut(index).expect(&*format!(
            "index {} out of board range 0..{}!",
            index,
            self.width * self.height
        ));
        *change = tile;
    }

    pub fn at(&self, pos: Position) -> Disc {
        let index = self.pos(pos);
        *self.tiles.get(index).expect(&*format!(
            "index {} out of board range 0..{}!",
            index,
            self.width * self.height
        ))
    }

    fn pos(&self, pos: Position) -> usize {
        (pos.row * self.width as isize + pos.col) as usize
    }

    pub fn len(&self) -> usize {
        self.width * self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new(8, 8);
        // Middle tiles: (3, 3), (3, 4), (4, 3), (3, 3)
        //     as index:     27,     28,     35,     36
        //        tiles:  black,  white,  white,  black
        board
            .black(Position { row: 3, col: 3 })
            .white(Position { row: 3, col: 4 })
            .white(Position { row: 4, col: 3 })
            .black(Position { row: 4, col: 4 });

        board
    }
}

fn display_board_top(f: &mut std::fmt::Formatter<'_>, width: usize) -> std::fmt::Result {
    write!(f, "┌")?;
    for _ in 0..(width - 1) {
        write!(f, "───┬")?;
    }
    writeln!(f, "───┐")
}

fn display_board_row(f: &mut std::fmt::Formatter<'_>, width: usize) -> std::fmt::Result {
    write!(f, "├")?;
    for _ in 0..(width - 1) {
        write!(f, "───┼")?;
    }
    writeln!(f, "───┤")
}

fn display_board_bottom(f: &mut std::fmt::Formatter<'_>, width: usize) -> std::fmt::Result {
    write!(f, "└")?;
    for _ in 0..(width - 1) {
        write!(f, "───┴")?;
    }
    writeln!(f, "───┘")
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_board_top(f, self.width)?;
        for row in 0..self.height {
            write!(f, "│")?;
            for col in 0..self.width {
                let tile = self.at(Position {
                    row: row as isize,
                    col: col as isize,
                });
                write!(f, "{}│", tile)?;
            }
            writeln!(f, "")?;
            if row < self.height - 1 {
                display_board_row(f, self.width)?;
            }
        }
        display_board_bottom(f, self.width)?;
        Ok(())
    }
}
