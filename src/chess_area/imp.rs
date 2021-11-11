use gtk::cairo::ImageSurface;
use gtk::{glib, DrawingArea};
use gtk::subclass::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Mutex;
use crate::board::Board;
use crate::board::piece::Piece;

pub struct ChessArea {
    pub board: Mutex<crate::Board>,
    pub texture: ImageSurface,
}

impl Default for ChessArea {
    fn default() -> Self {
        let mut file = File::open("chess_50.png").unwrap();
        let s = ImageSurface::create_from_png(&mut file).unwrap();
        Self {
            board: Mutex::new(
                Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            ),
            texture: s,
        }
    }
}


#[glib::object_subclass]
impl ObjectSubclass for ChessArea {
    const NAME: &'static str = "ChessArea";
    type Type = super::ChessArea;
    type ParentType = DrawingArea;
}

impl ObjectImpl for ChessArea {}

impl WidgetImpl for ChessArea {}

impl DrawingAreaImpl for ChessArea {}

impl ChessArea {
    pub fn draw_board(&self) {
        let board_guard = self.board.lock().unwrap();
        println!("{:?}", *board_guard);
    }
}