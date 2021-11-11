mod imp;

use std::ops::Deref;

use cairo::Surface;
use glib::Object;
use gtk::prelude::{Cast, DrawingAreaExt};
use gtk::subclass::prelude::ObjectSubclassExt;
use gtk::{glib, DrawingArea};

use crate::board::piece::Piece;
use crate::board::PieceRecord;

glib::wrapper! {
    pub struct ChessArea(ObjectSubclass<imp::ChessArea>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ChessArea {
    pub fn new() -> Self {
        let this: ChessArea = Object::new(&[]).expect("Failed to create `ChessArea`.");
        this.set_content_width(400);
        this.set_content_height(400);
        this.set_draw_func(Self::draw_self);
        this
    }

    fn draw_self(area: &DrawingArea, ctx: &gtk::cairo::Context, _a: i32, _b: i32) {
        let this: &Self = area.downcast_ref().unwrap();
        let inner = imp::ChessArea::from_instance(this);
        for row in 0..8 {
            for col in 0..8 {
                ctx.rectangle(col as f64 * 50.0, row as f64 * 50.0, 50.0, 50.0);
                if row % 2 != col % 2 {
                    ctx.set_source_rgb(0.3, 0.1, 0.1);
                } else {
                    ctx.set_source_rgb(1.0, 1.0, 1.0);
                }
                ctx.fill().unwrap();
            }
        }

        ctx.set_font_size(50.0);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        let surf = inner.texture.deref();

        let board_ref = inner.board.lock().unwrap();
        for fig in &board_ref.pieces {
            Self::draw_figure(ctx, fig, surf);
        }
    }

    fn draw_figure(ctx: &gtk::cairo::Context, figure: &PieceRecord, surface: &Surface) {
        let col = figure.pos.1 as f64 * 50.0;
        let row = (7 - figure.pos.0) as f64 * 50.0;
        let offsets = Self::get_piece_offsets(&figure.piece);
        ctx.set_source_surface(surface, col - offsets.0, row - offsets.1)
            .unwrap();
        ctx.rectangle(col, row, 50.0, 50.0);
        ctx.fill().unwrap();
    }
    
    fn get_piece_offsets(piece: &Piece) -> (f64, f64) {
        let x_offset = match piece.kind {
            crate::board::piece::PieceKind::Pawn => 5.0,
            crate::board::piece::PieceKind::Bishop => 2.0,
            crate::board::piece::PieceKind::Knight => 3.0,
            crate::board::piece::PieceKind::Rook => 4.0,
            crate::board::piece::PieceKind::King => 0.0,
            crate::board::piece::PieceKind::Queen => 1.0,
        };

        let y_offset = match piece.color {
            crate::board::piece::PieceColor::Black => 1.0,
            crate::board::piece::PieceColor::White => 0.0,
        };
        (x_offset * 50.0, y_offset * 50.0)
    }
}
