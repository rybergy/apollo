use clap::ArgMatches;

use std::io::{self, Write};

use crate::{othello, solve::*};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{self, Color},
    terminal::{self, ClearType},
    Result,
};
use othello::{Board, Disc, Othello, Position};

struct OthelloPlayer {
    w: io::Stdout,
    position: Position,
    game: Othello,
    player: Disc,
    solve: Box<dyn Search>,
}

impl OthelloPlayer {
    pub fn new(board: Board) -> Self {
        OthelloPlayer {
            w: io::stdout(),
            position: Position::new(0, 0),
            game: Othello::new(board),
            player: Disc::Black,
            solve: AlphaBetaOrdering::new(HWeightedMobility::new()),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        execute!(self.w, terminal::EnterAlternateScreen, cursor::Hide)?;
        terminal::enable_raw_mode()?;

        execute!(
            self.w,
            style::ResetColor,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All),
            cursor::Hide,
            style::SetForegroundColor(Color::Black),
            style::SetBackgroundColor(Color::DarkGreen)
        )?;

        print(&mut self.w, format!("{}", self.game.board()))?;

        self.update_board()?;

        self.position.row = 0;
        self.position.col = 0;

        self.move_cursor(0, 0)
    }

    pub fn clean(&mut self) -> Result<()> {
        execute!(
            self.w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()
    }

    fn disc_color(&self, disc: Disc) -> Color {
        match disc {
            Disc::Black => Color::Black,
            Disc::White => Color::White,
            Disc::Empty => Color::DarkGreen,
        }
    }

    fn move_cursor(&mut self, row: isize, col: isize) -> Result<()> {
        let disc = self.game.board().at(self.position);
        self.write_color(self.position, self.disc_color(disc))?;

        self.position.row = row;
        self.position.col = col;
        self.bind_position();

        let color = if self.game.is_valid_move(self.position, self.player) {
            Color::Blue
        } else {
            Color::Red
        };

        self.write_color(self.position, color)?;

        self.set_status(format!(
            "Coordinates: ({}, {})",
            self.position.row, self.position.col
        ))
    }

    fn cursor_position(&self, position: Position) -> (u16, u16) {
        (2 + 2 * position.row as u16, 1 + 4 * position.col as u16)
    }

    fn write_color(&mut self, position: Position, color: Color) -> Result<()> {
        let (r, c) = self.cursor_position(position);
        execute!(
            self.w,
            cursor::MoveTo(c, r),
            style::SetBackgroundColor(color),
            style::Print("   ")
        )
    }

    fn bind_position(&mut self) {
        if self.position.col < 0 {
            self.position.col = 0;
        } else if self.position.col >= self.game.board().width() as isize {
            self.position.col = self.game.board().width() as isize - 1;
        }

        if self.position.row < 0 {
            self.position.row = 0;
        } else if self.position.row >= self.game.board().height() as isize {
            self.position.row = self.game.board().height() as isize - 1;
        }
    }

    fn update_board(&mut self) -> Result<()> {
        for row in 0..self.game.board().height() {
            for col in 0..self.game.board().width() {
                let pos = Position::new(row as isize, col as isize);
                let disc = self.game.board().at(pos);
                self.write_color(pos, self.disc_color(disc))?;
            }
        }
        Ok(())
    }

    fn attempt_place(&mut self) -> Result<()> {
        if self.game.is_valid_move(self.position, self.player) {
            self.game.place(self.position, self.player);
            self.update_board()?;
            self.move_cursor(0, 0)?;
            // Do the AI's moves
            while {
                self.other_move()?;
                
                self.game.valid_moves(self.player).len() == 0
            } {}
        } else {
            self.set_status("Invalid position!".into())?;
        }
        Ok(())
    }

    fn other_move(&mut self) -> Result<()> {
        let opposite = self.player.opponent();
        let (mv, _) = self.solve.search(&self.game, opposite, 4);
        if let Some(pos) = mv {
            self.game.place(pos, opposite);
        } else {
            self.set_status(String::from("No valid moves for computer"))?;
        }

        self.update_board()?;
        self.move_cursor(0, 0)?;
        Ok(())
    }

    pub fn looping(&mut self) -> Result<()> {
        loop {
            match read_char()? {
                'w' => self.move_cursor(self.position.row - 1, self.position.col)?,
                'a' => self.move_cursor(self.position.row, self.position.col - 1)?,
                's' => self.move_cursor(self.position.row + 1, self.position.col)?,
                'd' => self.move_cursor(self.position.row, self.position.col + 1)?,
                ' ' => self.attempt_place()?,
                'q' => break,
                _ => (),
            }
        }
        Ok(())
    }

    pub fn set_status(&mut self, text: String) -> Result<()> {
        let status_line = 2 + 2 * self.game.board().height();
        execute!(
            self.w,
            cursor::MoveTo(0, status_line as u16),
            style::ResetColor,
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::SetBackgroundColor(Color::Black),
            style::SetForegroundColor(Color::White),
            style::Print(text)
        )
    }
}

pub fn main(_: &ArgMatches) -> Result<()> {
    let mut player = OthelloPlayer::new(Board::default());
    player.init()?;
    player.looping()?;
    player.clean()?;
    Ok(())
}

fn print<W: Write>(w: &mut W, s: String) -> Result<()> {
    for line in s.lines() {
        execute!(w, cursor::MoveToNextLine(1), style::Print(line),)?
    }
    Ok(())
}

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}
