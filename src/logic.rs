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

//NOTE: We can do something cute to solve this. If we return a result, we can return either a grid
//or we return an error, saying there has been some kind of conflict.
fn _backtrack(
    sudoku: Vec<Vec<SudokuCell>>,
) -> Result<Vec<Vec<SudokuCell>>, crate::logic::ValidityError> {
    if true {
        return Err(ValidityError::IsInvalid);
    }
    Ok(sudoku)

    //TODO: If Err(ValidityError::IsInvalid) is returned, it means that entering that a specific
    //value on a specific location means that in its row, column or box that value already exists.
}
