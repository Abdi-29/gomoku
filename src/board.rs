pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Option<bool>>>,
    pub current_player: bool,
}

pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cells: vec![vec![None; size]; size],
            current_player: true,
        }
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
