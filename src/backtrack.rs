use crate::is_valid;
use crate::InvalidError;
use crate::SudokuCell;

pub fn backtrack(mut sudoku: Vec<Vec<SudokuCell>>) -> Result<Vec<Vec<SudokuCell>>, InvalidError> {
    //We check first if the entire grid is filled with values.
    //If it is not, we take the store the first empty value.
    let mut is_filled = true;
    let mut row: usize = 0;
    let mut column: usize = 0;
    for (line_index, line) in sudoku.clone().into_iter().enumerate() {
        for (cell_index, cell) in line.into_iter().enumerate() {
            if cell.value == 0 {
                is_filled = false;
                row = line_index;
                column = cell_index;
                break;
            }
        }
        if !is_filled {
            break;
        }
    }

    //If is_filled is true, we know that all values are filled in, meaning that the sudoku is
    //solved

    if is_filled {
        return Ok(sudoku);
    }
    for value in 1..=9 {
        if is_valid(&sudoku, row, column, value) {
            sudoku[row][column].value = value;
            match backtrack(sudoku) {
                Ok(x) => return Ok(x),
                Err(InvalidError(e)) => sudoku = e,
            }
            sudoku[row][column].value = 0;
        }
    }
    Err(InvalidError(sudoku))
}
