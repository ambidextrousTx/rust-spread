extern crate eframe;
use eframe::egui;

#[derive(Default)]
pub struct SpreadsheetApp {
    rows: usize,
    cols: usize,
    num_tabs: usize,
    pub cells: Vec<Vec<Vec<String>>>,
    pub tabs: Vec<String>,
    selected_tab: usize
}

impl eframe::App for SpreadsheetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (i, tab_title) in self.tabs.iter().enumerate() {
                    if ui.selectable_label(self.selected_tab == i, tab_title).clicked() {
                        self.selected_tab = i;
                    }
                }

                ui.add_space(8.0);
                if ui.button("➕").clicked() {
                    let tab_num = self.tabs.len() + 1;
                    self.tabs.push(format!("Sheet {}", tab_num));
                    self.cells.push(vec![vec![String::new(); self.cols]; self.rows]);
                    self.selected_tab = self.tabs.len() - 1; // Switch to new tab
                }
            });

            ui.separator();

            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("Sheet 1")
                .striped(true)
                .show(ui, |ui| {
                    for tab in 0..self.num_tabs {
                        for row in 0..self.rows {
                            for col in 0..self.cols {
                                ui.text_edit_singleline(&mut self.cells[tab][row][col]);
                            }
                            ui.end_row();
                        }
                    }
                });
            });
        });
    }
}

impl SpreadsheetApp {
    pub fn new(rows: usize, cols: usize, num_tabs: usize) -> Self {
        Self {
            rows,
            cols,
            num_tabs,
            cells: vec![vec![vec![String::new(); cols]; rows]],
            tabs: vec!["Sheet 1".to_owned()],
            selected_tab: 0
        }
    }

    pub fn get_cell(&self, tab: usize, row: usize, col: usize) -> Option<&str> {
        self.cells.get(tab)
            .and_then(|sheet| sheet.get(row))
            .and_then(|row| row.get(col))
            .map(|cell| cell.as_str())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cell() {
        let mut app = SpreadsheetApp::default();
        app.cells = vec![vec![
            vec!["A1".to_string(), "B1".to_string()],
            vec!["A2".to_string(), "B2".to_string()],
        ]];

        assert_eq!(app.get_cell(0, 0, 0), Some("A1"));
        assert_eq!(app.get_cell(0, 0, 1), Some("B1"));
        assert_eq!(app.get_cell(0, 2, 0), None); // Out of bounds
    }
}

