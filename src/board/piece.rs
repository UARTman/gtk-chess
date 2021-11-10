use std::fmt::{Display, Formatter, Write, Debug};

#[derive(Copy, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

#[derive(Copy, Clone, Debug)]
pub enum PieceKind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
}

#[derive(Copy, Clone, Debug)]
pub enum PieceColor {
    Black,
    White
}

impl Piece {
    pub fn from_notation(c: char) -> Self {
        use PieceKind::*;
        use PieceColor::*;
        Self {
            kind: match c.to_lowercase().next().unwrap() {
                'p' => Pawn,
                'r' => Rook,
                'n' => Knight,
                'b' => Bishop,
                'k' => King,
                'q' => Queen,
                _ => panic!("Invalid piece code!"),
            },
            color: match c.is_lowercase()  {
                true => Black,
                false => White,
            }
        }
    }
}


impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let character = match self.kind {
            PieceKind::Pawn => 'p',
            PieceKind::Bishop => 'b',
            PieceKind::Knight => 'n',
            PieceKind::Rook => 'r',
            PieceKind::King => 'k',
            PieceKind::Queen => 'q',
        };

        let final_character = match self.color {
            PieceColor::Black => character,
            PieceColor::White => character.to_uppercase().next().unwrap(),
        };

        f.write_char(final_character)
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.color, f)?;
        f.write_char(' ')?;
        Debug::fmt(&self.kind, f)?;
        Ok(())
    }
}