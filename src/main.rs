mod board;
mod chess_area;
use board::Board;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use crate::chess_area::ChessArea;

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

    window.set_child(Some(&area));

    window.present();
}

