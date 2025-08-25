pub struct Board {
    size: usize,
    cells: [[Option<bool>]],
    current_player: bool
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cells: vec![vec![None; size]],
            current_player: true
        }
    }

    pub fn place_stone(&mut self, x: usize, y: usize) {
        if (x < self.size && y < self.size
            && self.cells[y][x].is_empty()) {
            self.cells[y][x] = self.current_player;
            self.current_player = !self.current_player;
        }
    }
}