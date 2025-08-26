mod board;
mod rendering;
mod input;

use macroquad::prelude::*;
use board::Board;
use rendering::draw_board;
use input::handle_input;

const BOARD_SIZE: usize = 5;
const CELL_SIZE: f32 = 70.0;

#[macroquad::main("Gomoku")]
async fn main() {
    let mut board = Board::new(BOARD_SIZE);
    let texture: Texture2D = load_texture("images/wood.png").await.unwrap();
    let white_stone: Texture2D = load_texture("images/white.png").await.unwrap();
    let black_stone: Texture2D = load_texture("images/black.png").await.unwrap();

    texture.set_filter(FilterMode::Linear);
    white_stone.set_filter(FilterMode::Linear);
    black_stone.set_filter(FilterMode::Linear);

    loop {
        clear_background(GRAY);
        
        draw_board(&board, &texture, &white_stone, &black_stone, CELL_SIZE);
        if let Some((x, y)) = handle_input(CELL_SIZE, BOARD_SIZE) {
            print!("x: {x} and y: {y}\n");
            board.place_stone(x, y);
        }
        next_frame().await
    }
} 