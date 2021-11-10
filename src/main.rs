mod board;
mod chess_area;
use board::Board;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use crate::chess_area::ChessArea;
use std::convert::TryInto;

fn main() {
    let app = Application::builder()
        .application_id("site.uartman.chess")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("Chess").build();

    let area = ChessArea::new();
    area.set_content_width(500);
    area.set_content_height(500);

    window.set_child(Some(&area));

    window.present();
}

