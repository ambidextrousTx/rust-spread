extern crate eframe;
use eframe::egui;
mod app;
use app::SpreadsheetApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "rust-spread",
        options,
        Box::new(|_cc| Ok(Box::new(SpreadsheetApp::default()))),
    )
}
