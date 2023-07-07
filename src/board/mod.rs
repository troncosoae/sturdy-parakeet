
// todo make private
pub enum CellState {
    Empty,
    Player1,
    Player2,
}

// todo make private
pub struct Cell {
    pub state: CellState,
}


pub struct Board {
    pub cells: Vec<Vec<Cell>>,
}
