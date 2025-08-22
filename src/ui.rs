use eframe::egui;
use egui::StrokeKind;


#[derive(Default)]
pub struct GomokuApp {
    board: [[Option<bool>; 19]; 19],
    current_player: bool,
}

impl eframe::App for GomokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("Gomoku");

            let cell_size = 19.0;

            // Draw the board
            for y in 0..19 {
                for x in 0..19 {
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(cell_size, cell_size),
                        egui::Sense::click(),
                    );

                    // Background
                    let color = match self.board[y][x] {
                        Some(true) => egui::Color32::BLACK,
                        Some(false) => egui::Color32::WHITE,
                        None => egui::Color32::RED,
                    };

                    ui.painter().rect_filled(rect, 2.0, color);
                    ui.painter().rect_stroke(rect, 12.0, egui::Stroke::new(1.0, egui::Color32::RED), StrokeKind::Outside);

                    if response.clicked() && self.board[y][x].is_none() {
                        self.board[y][x] = Some(self.current_player);
                        self.current_player = !self.current_player;
                    }
                }
                ui.end_row();
            }
        });
    }
}
