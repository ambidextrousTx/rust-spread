extern crate eframe;
use eframe::egui;

#[derive(Default)]
pub struct SpreadsheetApp {
    rows: usize,
    cols: usize,
    pub cells: Vec<Vec<String>>,
}

impl eframe::App for SpreadsheetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.visuals_mut().selection.bg_fill = ui.visuals_mut().window_fill();
                ui.selectable_label(true, "Sheet 1")
            });

            ui.separator();

            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("Sheet 1")
                .striped(true)
                .show(ui, |ui| {
                    for row in 0..self.rows {
                        for col in 0..self.cols {
                            ui.text_edit_singleline(&mut self.cells[row][col]);
                        }
                        ui.end_row();
                    }
                });
            });
        });
    }
}

impl SpreadsheetApp {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            cells: vec![vec![String::new(); cols]; rows],
        }
    }

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

