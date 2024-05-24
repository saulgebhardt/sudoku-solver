use crate::cell::SudokuCell;

/*
* This file contains the logic for the sudoku solver program.
* Currently, it only contains an is_valid function, which provides info wheter it is a legal move
* to place a value at a specific coordinate.
*/

enum ValidityError {
    IsInvalid,
}

//WARNING: Maybe we need to borrow value as well, as we then do not lose it. If we do, we also need
//to borrow the Cell in the vec itself as well.

fn is_valid(sudoku: &Vec<Vec<SudokuCell>>, row: usize, column: usize, value: u8) -> bool {
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

fn backtrack(
    mut sudoku: Vec<Vec<SudokuCell>>,
) -> Result<Vec<Vec<SudokuCell>>, crate::logic::ValidityError> {
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

    //FIX: The problem here is that it is not possible to fill in a value temporarily and call it
    //recursively to backtrack. However, then it owns the value, and we do not return it.

    /*    for value in 1..=9 {
            if is_valid(&sudoku, row, column, value) {
                sudoku[row][column].value = value;
                if let Ok(x) = backtrack(sudoku) {
                    return Ok(x);
                } else {
                    x[row][column].value = 0;
                }
            }
       }
    */
    Err(ValidityError::IsInvalid)
}
