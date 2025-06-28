extern crate eframe;
use eframe::egui;

#[derive(Default)]
pub struct SpreadsheetApp {
    pub cells: Vec<Vec<String>>,
}

impl eframe::App for SpreadsheetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Your spreadsheet UI code here
        });
    }
}

impl SpreadsheetApp {
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&str> {
        self.cells.get(row).and_then(|r| r.get(col)).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cell() {
        let mut app = SpreadsheetApp::default();
        app.cells = vec![
            vec!["A1".to_string(), "B1".to_string()],
            vec!["A2".to_string(), "B2".to_string()],
        ];

        assert_eq!(app.get_cell(0, 0), Some("A1"));
        assert_eq!(app.get_cell(1, 1), Some("B2"));
        assert_eq!(app.get_cell(2, 0), None); // Out of bounds
    }
}

