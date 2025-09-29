use std::collections::HashSet;
use rand::prelude::*;

use crate::board::{Board, Delta, Position};

pub fn best_move(board: &Board) -> Option<Position> {
    let moves = generate_valid_moves(board);
    if moves.is_empty() {
        return None;
    }

    let mut best_score = i32::MIN;
    let mut best_moves = Vec::new();
    let mut rng = rand::rng();

    for &pos in &moves {
        let mut sim_board = board.clone();
        sim_board.place_stone(pos);
        let score = minimax(&sim_board, 4, !board.current_player, pos); // Depth 2, opponent's turn, pass pos as last
        if score > best_score {
            best_score = score;
            best_moves.clear();
            best_moves.push(pos);
        } else if score == best_score {
            best_moves.push(pos);
        }
    }

    println!("best_move {:#?}", best_moves);
    best_moves.choose(&mut rng).copied()
}

fn generate_valid_moves(board: &Board) -> Vec<Position> {
    let mut moves = HashSet::new();
    let directions = [
        Delta { dx: -1, dy: -1 },
        Delta { dx: -1, dy: 0 },
        Delta { dx: -1, dy: 1 },
        Delta { dx: 0, dy: -1 },
        Delta { dx: 0, dy: 1 },
        Delta { dx: 1, dy: -1 },
        Delta { dx: 1, dy: 0 },
        Delta { dx: 1, dy: 1 },
    ];

    let mut has_occupained = false;

    for y in 0..board.size {
        for x in 0..board.size {
            if board.get_cell(Position { x, y }).is_some() {
                has_occupained = true;
                let pos = Position { x, y };
                for &dir in &directions {
                    if let Some(neighbor) = pos + dir {
                        if neighbor.is_valid(board.size) && board.get_cell(neighbor).is_none() {
                            moves.insert(neighbor);
                        }
                    }
                }
            }
        }
    }

    if !has_occupained {
        let center = Position{x: board.size / 2,  y: board.size / 2};
        moves.insert(center);

        for &dir in &directions {
            if let Some(neighbor) = center + dir {
                if neighbor.is_valid(board.size) && board.get_cell(neighbor).is_none() {
                    moves.insert(neighbor);
                }
            }
        }
    }

    moves.into_iter().collect()
}

fn minimax(board: &Board, depth: u32, player: bool, last_pos: Position) -> i32 {
    if depth == 0 || board.check_winner(last_pos).is_some() || board.is_board_full() {
        return evaluate(board, board.current_player, last_pos);
    }

    let moves = generate_valid_moves(board);
    if moves.is_empty() {
        return evaluate(board, board.current_player, last_pos);
    }

    if player == board.current_player {
        let mut max_score = i32::MIN;
        for pos in moves {
            let mut sim_board = board.clone();
            sim_board.place_stone(pos);
            let score = minimax(&sim_board, depth - 1, !player, pos); // Pass pos as new last_pos
            max_score = max_score.max(score);
        }
        max_score
    } else {
        let mut min_score = i32::MAX;
        for pos in moves {
            let mut sim_board = board.clone();
            sim_board.place_stone(pos);
            let score = minimax(&sim_board, depth - 1, !player, pos); // Pass pos as new last_pos
            min_score = min_score.min(score);
        }
        min_score
    }
}

fn evaluate(board: &Board, player: bool, last_pos: Position) -> i32 {
    let mut score = 0;
    let directions = [
        Delta { dx: 1, dy: 0 },
        Delta { dx: 0, dy: 1 },
        Delta { dx: 1, dy: 1 },
        Delta { dx: 1, dy: -1 },
    ];

    for dir in directions {
        let pos_count = board.count_dir(last_pos, dir.dx, dir.dy, player);
        let neg_count = board.count_dir(last_pos, -dir.dx, -dir.dy, player);
        score += (pos_count + neg_count) * 10;

        let opp_count = board.count_dir(last_pos, dir.dx, dir.dy ,!player) +
                        board.count_dir(last_pos, -dir.dx, -dir.dy, !player);
        score -= opp_count * 10;
    }
    score.try_into().unwrap()
}