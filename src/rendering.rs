use macroquad::prelude::*;
use crate::board::Board;

pub fn draw_board(board: &Board, texture: &Texture2D, cell_size: f32) {
    let size = board.size as f32;
    let offset_x = (screen_width() - cell_size * (size - 1.0)) / 2.0;
    let offset_y = (screen_height() - cell_size * (size - 1.0)) / 2.0;
    // wood texture
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
    // grid draw
    for i in 0..board.size {
        let x = offset_x + i as f32 * cell_size;
        let y = offset_y + i as f32 * cell_size;
        draw_line(offset_x, y, offset_x + (size - 1.0) * cell_size, y, 2.0, BLACK);
        draw_line(x, offset_y, x, offset_y + (size - 1.0) * cell_size, 2.0, BLACK);
    }
    // stones draw 
    // TODO implement 3d rendering for the stone so implement a camera
    for y in 0..board.size {
        for x in 0..board.size {
            if let Some(player) = board.cells[y][x] {
                let color = if player { BLACK } else { WHITE };
                let center_x = offset_x + x as f32 * cell_size;
                let center_y = offset_y + y as f32 * cell_size;
                draw_circle(center_x, center_y, cell_size * 0.4, color);
            }
        }
    }
}