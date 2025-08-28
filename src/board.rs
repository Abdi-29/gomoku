use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

#[derive(Clone, Copy, Debug)]
pub struct Delta {
    pub dx: isize,
    pub dy: isize
}

impl Position{
    pub fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }

    pub fn valid_pos(&self, size: usize) -> bool {
        self.x < size && self.y < size
    }
}

impl Add<Delta> for Position {
    type Output = Option<Position>;

    fn add(self, rhs: Delta) -> Self::Output {
        let x = self.x as isize + rhs.dx;
        let y = self.y as isize + rhs.dy;

        if x > 0 && y > 0 {
            Some(Position { x: x as usize, y: y as usize })
        } else {
            None
        }
    }
}

pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Option<bool>>>,
    pub current_player: bool,
    pub black_capture: usize,
    pub white_capture: usize
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cells: vec![vec![None; size]; size],
            current_player: true,
            black_capture: 0,
            white_capture: 0
        }
    }

    pub fn get_cell(&self, pos: Position) -> Option<bool> {
        if pos.valid_pos(self.size) {
            self.cells[pos.y][pos.x]
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, pos: Position, value: Option<bool>) {
        if pos.valid_pos(self.size) {
            self.cells[pos.y][pos.x] = value;
        }
    }

    pub fn is_board_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }

    pub fn place_stone(&mut self, x: usize, y: usize) {
        if x < self.size && y < self.size && self.cells[y][x].is_none() {
            self.cells[y][x] = Some(self.current_player);
            self.current_player = !self.current_player;
        }
    }

    pub fn check_winner(&self, pos: Position) -> Option<bool> {
        if let Some(player) = self.cells[pos.y][pos.x] {
            let directions = [
            (0, 1),
            (1, 0),
            (1, 1),
            (1, -1)
            ];

            for &(dx, dy) in &directions {
                let mut count = 1;
                count += self.count_dir(pos.x, pos.y, dx, dy, player);
                count += self.count_dir(pos.x, pos.y, -dx, -dy, player);

                if count >= 3 {
                    return Some(player);
                }
            }
        }
        None
    }

    fn count_dir(&self, mut x: usize, mut y: usize, dx: isize, dy: isize, player: bool) -> usize {
        let mut count = 0;
        loop {
            if dx < 0 && x == 0 || dy < 0 && y == 0 {
                break;
            }

            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;
            if x >= self.size || y >= self.size {
                break;
            } else if self.cells[y][x] != Some(player) {
                break;
            }
            count += 1;
        }
        count
    }
}
