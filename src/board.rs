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

    pub fn is_valid(&self, size: usize) -> bool {
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

    pub fn is_valid_move(&self, pos: Position) -> bool {
        if !pos.is_valid(self.size) || self.get_cell(pos).is_some() {
            return false;
        }
        true
    }

    pub fn place_stone(&mut self, pos: Position) {
        if !self.is_valid_move(pos) {
            return;
        }
        self.set_cell(pos, Some(self.current_player));
        self.capture(pos, self.current_player);
        self.current_player = !self.current_player;
    }

    pub fn check_winner(&self, pos: Position) -> Option<bool> {
        if self.black_capture >= 10 {
            return Some(true);
        }
        if self.white_capture >= 10 {
            return Some(false);
        }

        if let Some(player) = self.get_cell(pos) {
            let dirs = [
                Delta {dx: 1, dy: 0},
                Delta {dx: 0, dy: 1},
                Delta {dx: 1, dy: 1},
                Delta {dx: 1, dy: -1}
            ];

            for dir in dirs {
                let mut count = 1;
                count += self.count_dir(pos, dir, player);
                let neg_dir = Delta { dx: -dir.dx, dy: -dir.dy };
                count += self.count_dir(pos, neg_dir, player);

                if count >= 3 {
                    return Some(player);
                }
            }
        }
        None
    }

    fn count_dir(&self, s_pos: Position, dir: Delta, player: bool) -> usize {
        let mut count = 0;
        let mut curr_pos = s_pos;
        loop {
            if let Some(n_pos) = curr_pos + dir {
                if !n_pos.is_valid(self.size) {
                    break;
                }
                if self.get_cell(n_pos) == Some(player) {
                    count += 1;
                    curr_pos = n_pos;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        count
    }

    pub fn capture(&mut self, pos: Position, player: bool) {
        let turn = !player;
        let dirs = [
            Delta {dx: 1, dy: 0},
            Delta {dx: 0, dy: 1},
            Delta {dx: 1, dy: 1},
            Delta {dx: 1, dy: -1}
        ];

        for dir in dirs {
            for sign in [1, -1] {
                let tmp_dir = Delta { dx: dir.dx * sign, dy: dir.dy * sign };
                if let (Some(p1), Some(p2), Some(p3)) = (
                    pos + tmp_dir,
                    pos + Delta { dx: tmp_dir.dx * 2, dy: tmp_dir.dy * 2 },
                    pos + Delta { dx: tmp_dir.dx * 3, dy: tmp_dir.dy * 3 },
                ) {
                    if p1.is_valid(self.size) && p2.is_valid(self.size) && p3.is_valid(self.size)
                        && self.get_cell(p1) == Some(turn)
                        && self.get_cell(p2) == Some(turn)
                        && self.get_cell(p3) == Some(player) {
                        self.set_cell(p1, None);
                        self.set_cell(p2, None);
                        if player {
                            self.black_capture += 2;
                        } else {
                            self.white_capture += 2;
                        }
                    }
                }
            }
        }
    }
}
