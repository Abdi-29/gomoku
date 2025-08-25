use eframe::egui;
use egui::{pos2, Color32, Painter, Pos2, Rect, Response, Rounding, Sense, Stroke, Vec2};

#[derive(Default)]
pub struct GomokuApp {
    board: [[Option<bool>; 19]; 19], // true = black, false = white
    current_player: bool, // true = black starts
}

impl eframe::App for GomokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let board_size = 400.0;
        let n = 19usize;
        let cell_size = board_size / ((n - 1) as f32); // Correct spacing for 19 lines/19 positions

        // Allocate space for the board and get a response for interaction
        let (response, painter) = ui.allocate_painter(Vec2::new(board_size, board_size), Sense::click());

        // Draw wooden-like background (brown color; replace with Image if you load a texture)
        painter.rect_filled(response.rect, Rounding::ZERO, Color32::from_rgb(181, 136, 99));

        // Draw grid lines
        let stroke = Stroke::new(1.0, Color32::BLACK);
        for i in 0..n {
            let offset = (i as f32) * cell_size;
            // Horizontal lines
            painter.hline(0.0..=board_size, offset, stroke);
            // Vertical lines
            painter.vline(offset, 0.0..=board_size, stroke);
        }

        // Draw stones as circles
        for y in 0..n {
            for x in 0..n {
                if let Some(player) = self.board[y][x] {
                    let center = pos2(
                        (x as f32) * cell_size,
                        (y as f32) * cell_size,
                    );
                    let color = if player { Color32::BLACK } else { Color32::WHITE };
                    painter.circle_filled(center, cell_size / 2.0 * 0.9, color);
                    // Optional: Add a subtle shadow or outline for realism
                    painter.circle_stroke(center, cell_size / 2.0 * 0.9, Stroke::new(1.0, Color32::from_gray(50)));
                }
            }
        }

        // Handle clicks
        if response.clicked() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                // Convert click position to board coordinates (round to nearest for better snapping)
                let local_pos = pointer_pos - response.rect.left_top();
                let x = (local_pos.x / cell_size).round() as usize;
                let y = (local_pos.y / cell_size).round() as usize;
                if x < n && y < n && self.board[y][x].is_none() {
                    self.board[y][x] = Some(self.current_player);
                    self.current_player = !self.current_player;
                }
            }
        }
    });
}
    }