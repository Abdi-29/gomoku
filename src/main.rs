mod board;
mod rendering;
mod input;

use macroquad::prelude::*;
use board::Board;
use rendering::draw_board;
use input::handle_input;

use crate::board::Position;

const BOARD_SIZE: usize = 15;
const CELL_SIZE: f32 = 70.0;

#[derive(PartialEq)]
enum GameState {
    Ongoing,
    Over(Option<bool>)
}

#[macroquad::main("Gomoku")]
async fn main() {
    let mut board = Board::new(BOARD_SIZE);
    let texture: Texture2D = load_texture("images/wood.png").await.unwrap();
    let white_stone: Texture2D = load_texture("images/white.png").await.unwrap();
    let black_stone: Texture2D = load_texture("images/black.png").await.unwrap();

    texture.set_filter(FilterMode::Linear);
    white_stone.set_filter(FilterMode::Linear);
    black_stone.set_filter(FilterMode::Linear);

    let mut game_state = GameState::Ongoing;
    loop {
        clear_background(GRAY);
        
        draw_board(&board, &texture, &white_stone, &black_stone, screen_width() / 20.0, &game_state);
        
        match game_state {
            GameState::Ongoing => {
                if let Some((x, y)) = handle_input(screen_width() / 20.0, BOARD_SIZE) {
                    print!("x: {x} and y: {y}\n");
                    let pos = Position{x, y};
                    board.place_stone(x, y); // TODO refactor using Position
                    if let Some(winner) = board.check_winner(pos) {
                        game_state = GameState::Over(Some(winner));
                        println!("winner: {}", if winner {"Black"} else {"White"});
                        //TODO handle the game end on the gui
                    }
                }
            }
            GameState::Over(_) => {
                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::Ongoing;
                    board = Board::new(BOARD_SIZE);
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }
        next_frame().await
    }
}