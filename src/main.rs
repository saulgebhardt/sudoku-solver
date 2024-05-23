mod cell;
mod logic;

use cell::SudokuCell;
use prettytable::Table;

//NOTE: We need to store the values of the Cells in structs so we can keep track whether we are
//allowed to change values. Based on if the value was there from the start, we can also assign a
//color to it.
//
//NOTE: Can we confirm somehow that there exists only 1 solution? If there exist more than 1, the
//sudoku is invalid.

fn main() {
    //TODO: Read a single sudoku from a file, and parse it as a 2d array(Vec) of u8
    let temp_string = String::from(
        "050703060007000800000816000000030000005000100730040086906000204840572093000409000",
    );
    let sudoku_puzzle = create_sudoku(temp_string);
    print(&sudoku_puzzle);
}

fn create_sudoku(input_string: String) -> Vec<Vec<SudokuCell>> {
    let mut input_iterator = input_string.chars().into_iter();
    let mut sudoku_puzzle: Vec<Vec<SudokuCell>> = Vec::new();
    for _ in 0..9 {
        let mut line: Vec<SudokuCell> = Vec::new();
        for _ in 0..9 {
            let temp_u8 = input_iterator.next().unwrap();
            line.push(SudokuCell::new(temp_u8));
        }
        sudoku_puzzle.push(line);
    }
    sudoku_puzzle
}

fn print(sudoku: &Vec<Vec<SudokuCell>>) {
    let mut table = Table::new();
    for line in sudoku {
        table.add_row(line.into());
    }
    table.printstd();
}
//TODO: Implement some small timing tool, useful for assessing efficiency of algorithm and show
//paralellism.
