use crate::board::Board;
use crate::board::CellState;

pub struct Gui {
    pub board: Board,
}

impl Gui {
    pub fn print_board(self) {
        for row in self.board.cells {
            for cell in row {
                match cell.state {
                    CellState::Empty => println!("e"),
                    CellState::Player1 => println!("p1"),
                    CellState::Player2 => println!("p2"),
                }
            }
        }
    }
}



pub fn execute_game_flow(gui: Gui) {
    for iteration in 1..4 {
        println!(
            "iteration: {iter}",
            iter=iteration,
        );
    }
    
    gui.print_board();
}

