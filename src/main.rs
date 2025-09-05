extern crate eframe;
mod app;
use app::SpreadsheetApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "rust-spread",
        options,
        Box::new(|_cc| Ok(Box::new(SpreadsheetApp::new(50, 50, 1)))),
    )
}
