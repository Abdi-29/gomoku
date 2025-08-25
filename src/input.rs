use macroquad::prelude::*;

pub fn handle_input(cell_size: f32, board_size: usize) -> Option<(usize, usize)> {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        let offset_x = (screen_width() - cell_size * (board_size as f32 - 1.0)) / 2.0;
        let offset_y = (screen_height() - cell_size * (board_size as f32 - 1.0)) / 2.0;
        let gx = ((mx - offset_x) / cell_size).round() as i32;
        let gy = ((my - offset_y) / cell_size).round() as i32;
        
        if gx >= 0 && gy >= 0 && gx < board_size as i32 && gy < board_size as i32 {
            return Some((gx as usize, gy as usize));
        }
    }
    None
}