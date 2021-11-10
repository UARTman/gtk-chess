mod piece;
use piece::*;
use std::fmt::{Debug, Formatter, Write};

pub struct Board {
    pub pieces: Vec<PieceRecord>,
}

impl Board {
    pub fn from_fen(fen: &str) -> Self {
        // Let's begin by selecting the board.
        let mut split_iterator = fen.split(' ');
        let fen_board = split_iterator.next().expect("FEN is incorrect!");


        let mut pieces: Vec<PieceRecord> = vec![];

        let mut row = 8;
        let mut col;

        for line in fen_board.split('/') {
            row -= 1;
            col = 0;
            for c in line.chars() {
                if let Ok(num) = c.to_string().parse::<u8>() {
                    col += num as usize;
                } else {
                    pieces.push(PieceRecord { piece: Piece::from_notation(c), pos: (row, col) });
                    col += 1;
                }
            }
        }

        // TODO: store other FEN information.

        Self {pieces}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PieceRecord {
    pub piece: Piece,
    pub pos: (usize, usize),
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in (0..8).rev() {
            f.write_char('|')?;
            for col in 0..8 {
                let mut empty = true;
                for record in &self.pieces {
                    if record.pos.0 == row && record.pos.1 == col {
                        f.write_fmt(format_args!("{}", record.piece))?;
                        empty = false;
                        break;
                    }
                }
                if empty {f.write_char(' ')?;};
                f.write_char('|')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}