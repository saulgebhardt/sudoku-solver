pub mod backtrack;
pub mod cell;
pub mod pencilmarks;

use crate::cell::SudokuCell;

pub struct InvalidError(Vec<Vec<SudokuCell>>);

pub fn is_valid(sudoku: &Vec<Vec<SudokuCell>>, row: usize, column: usize, value: u8) -> bool {
    for i in 0..8 {
        if sudoku[row][i].value == value {
            return false;
        }
    }
    for i in 0..8 {
        if sudoku[i][column].value == value {
            return false;
        }
    }
    let x_coord = (column / 3) * 3;
    let y_coord = (row / 3) * 3;
    for i in y_coord..y_coord + 3 {
        for j in x_coord..x_coord + 3 {
            if sudoku[i][j].value == value {
                return false;
            }
        }
    }
    true
}
