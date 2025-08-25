mod board;
mod rendering;
mod input;

use macroquad::prelude::*;
use board::Board;
use rendering::draw_board;
use input::handle_input;

const BOARD_SIZE: usize = 2;
const CELL_SIZE: f32 = 100.0;

#[macroquad::main("Gomoku")]
async fn main() {
    let mut board = Board::new(BOARD_SIZE);
    let texture: Texture2D = load_texture("images/wood.png").await.unwrap();
    texture.set_filter(FilterMode::Linear);

    loop {
        clear_background(WHITE);

        draw_board(&board, &texture, CELL_SIZE);
        if let Some((x, y)) = handle_input(CELL_SIZE, BOARD_SIZE) {
            board.place_stone(x, y);
        }
        next_frame().await
    }
} 