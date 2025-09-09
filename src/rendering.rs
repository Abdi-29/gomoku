use macroquad::prelude::*;
use crate::{board::Board, GameState};

pub fn draw_board(board: &Board, texture: &Texture2D, white_stone: &Texture2D, black_stone: &Texture2D, cell_size: f32, game_state: &GameState) {
    let size = board.size as f32;
    let offset_x = (screen_width() - cell_size * (size - 1.0)) / 2.0;
    let offset_y = (screen_height() - cell_size * (size - 1.0)) / 2.0;
    let board_width = (size - 1.0) * cell_size;
    let board_height = (size - 1.0) * cell_size;
    // wood texture
    draw_texture_ex(
        texture,
        offset_x,
        offset_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(board_width, board_height)),
            ..Default::default()
        },
    );
    // grid draw
    for i in 0..board.size {
        let x = offset_x + i as f32 * cell_size;
        let y = offset_y + i as f32 * cell_size;
        draw_line(offset_x, y, offset_x + (size - 1.0) * cell_size, y, 2.0, DARKBROWN);
        draw_line(x, offset_y, x, offset_y + (size - 1.0) * cell_size, 2.0, DARKBROWN);
    }
    // stones draw
    for y in 0..board.size {
        for x in 0..board.size {
            if let Some(player) = board.cells[y][x] {
                // let color = if player { BLACK } else { WHITE };
                let center_x = offset_x + x as f32 * cell_size - cell_size * 0.5;
                let center_y = offset_y + y as f32 * cell_size - cell_size * 0.5;
                let params = DrawTextureParams {
                    dest_size: Some(vec2(cell_size, cell_size)),
                    ..Default::default()
                };
                let color = if player { black_stone } else { white_stone };
                // draw_circle(center_x, center_y, cell_size * 0.4, color);
                draw_texture_ex(color, center_x, center_y, WHITE, params);
            }
        }
    }

    // Preview move
    if *game_state == GameState::Ongoing {
        let (mouse_x, mouse_y) = mouse_position();
        let grid_x = ((mouse_x - offset_x) / cell_size).round() as i32;
        let grid_y = ((mouse_y - offset_y) / cell_size).round() as i32;
    
        if grid_x >= 0 && grid_y >= 0 && grid_x < board.size as i32 && grid_y < board.size as i32 {
            if board.cells[grid_y as usize][grid_x as usize].is_none() {
                let preview_stone = if board.current_player {
                    black_stone
                } else {
                    white_stone
                };
    
                let center_x = offset_x + grid_x as f32 * cell_size - cell_size * 0.5;
                let center_y = offset_y + grid_y as f32 * cell_size - cell_size * 0.5;
    
                let params = DrawTextureParams {
                    dest_size: Some(vec2(cell_size, cell_size)),
                    ..Default::default()
                };
    
                draw_texture_ex(
                    preview_stone,
                    center_x,
                    center_y,
                    Color::new(1.0, 1.0, 1.0, 0.5),
                    params,
                );
            }
        }
    }

    let black_capt = format!("Black Captures: {}", board.black_capture);
    let white_capt = format!("White Captures: {}", board.white_capture);
    draw_text(&black_capt, 10.0, 20.0, 20.0, BLACK);
    draw_text(&white_capt, 10.0, 50.0, 20.0, WHITE);

    if let GameState::Over(winner) = game_state {
        let winner_text = match winner {
            Some(true) => "Black wins!",
            Some(false) => "White wins!",
            None => "It's a draw!"
        };

        // let promt_text = "Game Over. Press [enter] to play again or [Esc] to scape.";
        let font_size = 40.;
        let text_size = measure_text(winner_text, None, font_size as _, 1.0);

        draw_text(winner_text,
            (screen_width() - text_size.width) / 2.,
            (screen_height() - text_size.height) / 2. - 100.,
            font_size,
            BLACK,
        );


    }

}


//TODO preview the move before deciding
