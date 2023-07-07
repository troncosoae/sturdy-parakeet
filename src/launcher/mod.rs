use crate::gui::Gui;
use crate::gui;
use crate::board::Board;
// todo delete these uses?
use crate::board::Cell;
use crate::board::CellState;

pub fn run_launcher() {
    println!("runLauncher!");

    let mut board = star_game();
    let mut gui = start_gui(board);

    gui::execute_game_flow(gui);
    
}
fn star_game() -> Board {
    println!("startGame!");

    let board = Board {
        cells: vec![
            vec![
                Cell {state: CellState::Empty},
                Cell {state: CellState::Empty},
            ],
            vec![
                Cell {state: CellState::Empty},
                Cell {state: CellState::Empty},
            ],
        ]
    };

    board
}

fn start_gui(board: Board) -> Gui {
    println!("startGui");

    let gui = Gui {
        board: board,
    };

    gui
}
