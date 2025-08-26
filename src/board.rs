pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Option<bool>>>,
    pub current_player: bool,
}

// pub struct Square {
//     x: usize,
//     y: usize
// }

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

    // pub fn check_winner(&self, x: usize, y: usize) -> Option<bool> {
    //     let directions = [
    //         (0, 1),
    //         (1, 0),
    //         (1, 1),
    //         (1, -1)
    //     ];

    //     if let Some(player) = {

    //     }
    // }
}
