use gtk::{glib, DrawingArea};
use gtk::subclass::prelude::*;
use gtk::glib::subclass::types::InterfaceList;
use std::sync::Mutex;
use crate::board::Board;

pub struct ChessArea {
    pub board: Mutex<crate::Board>
}

impl Default for ChessArea {
    fn default() -> Self {
        Self {
            board: Mutex::new(
                Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            )
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