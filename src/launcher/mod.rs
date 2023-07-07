use crate::board::Board;
// todo delete these uses?
use crate::board::Cell;
use crate::board::CellState;

pub fn run_launcher() {
    println!("runLauncher!");

    star_game();
    start_gui();
}
fn star_game() -> Board
{
    println!("startGame!");

    let board = Board {
        cells: vec![
            Cell {state: CellState::Empty},
            Cell {state: CellState::Empty},
        ]
    };

    board
}

fn start_gui()
{
    println!("startGui");
}
