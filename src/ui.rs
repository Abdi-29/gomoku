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
            let cell_size = 20.0;

            egui::Grid::new("gomoku_board")
                .spacing([1.0, 6.0])
                .show(ui, |ui| {
                    for y in 0..19 {
                        for x in 0..19 {
                            let color = match self.board[y][x] {
                                Some(true) => egui::Color32::BLACK,
                                Some(false) => egui::Color32::WHITE,
                                None => egui::Color32::RED
                            };

                            let response = ui.add_sized(
                                [cell_size, cell_size],
                                egui::Button::new("").fill(color),
                            );

                            if response.clicked() && self.board[y][x].is_none() {
                                self.board[y][x] = Some(self.current_player);
                                self.current_player = !self.current_player;
                            }

                        }
                        ui.end_row();
                    }
                });
        });
    }
}