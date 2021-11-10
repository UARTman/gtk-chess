mod imp;

use glib::Object;
use gtk::prelude::{Cast, DrawingAreaExt};
use gtk::subclass::prelude::ObjectSubclassExt;
use gtk::{glib, DrawingArea};

glib::wrapper! {
    pub struct ChessArea(ObjectSubclass<imp::ChessArea>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ChessArea {
    pub fn new() -> Self {
        let this: ChessArea = Object::new(&[]).expect("Failed to create `ChessArea`.");
        this.set_content_width(500);
        this.set_content_height(500);
        this.set_draw_func(Self::draw_self);
        this
    }

    fn draw_self(area: &DrawingArea, ctx: &gtk::cairo::Context, _a: i32,_b: i32) {
        let this: &Self = area.dynamic_cast_ref().unwrap();
        let inner = imp::ChessArea::from_instance(this);
        for row in 0..8 {
            for col in 0..8 {
                ctx.rectangle(col as f64 * 50.0, row as f64 * 50.0, 50.0, 50.0);
                if row % 2 != col % 2 {
                    ctx.set_source_rgb(0.0, 0.0, 0.0);
                } else {
                    ctx.set_source_rgb(1.0, 1.0, 1.0);
                }
                ctx.fill().unwrap();
            }
        }

        ctx.set_font_size(50.0);
        ctx.set_source_rgb(1.0, 0.0, 0.0);

        let board_ref = inner.board.lock().unwrap();
        for fig in &board_ref.pieces {
            let row = fig.pos.0;
            let col = fig.pos.1;
            ctx.move_to(col as f64 * 50.0, (8-row) as f64 * 50.0);
            ctx.show_text(&format!("{}", fig.piece)).unwrap();
        }
    }
}
