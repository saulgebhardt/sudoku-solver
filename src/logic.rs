use crate::cell::Cell;

//WARNING: Maybe we need to borrow value as well, as we then do not lose it. If we do, we also need
//to borrow the Cell in the vec itself as well.
fn is_valid(sudoku: &Vec<Vec<Cell>>, row: usize, column: usize, value: u8) -> bool {
    //NOTE: Check the row condition
    for i in 0..8 {
        if sudoku[row][i].value == value {
            return false;
        }
    }
    //NOTE: Check the column condition
    for i in 0..8 {
        if sudoku[i][column].value == value {
            return false;
        }
    }
    //TODO: Check the box condition. This requires some arithmetic
    // First, we need to define the box in which we're in
    let x_coord = column / 3;
    let y_coord = row / 3;
    //TODO: Check every value in the box
    //FIX: These values might be the wrong way around
    let x_coord = x_coord * 3;
    let y_coord = y_coord * 3;
    for i in y_coord..y_coord + 3 {
        for j in x_coord..x_coord + 3 {
            if sudoku[i][j].value == value {
                return false;
            }
        }
    }
    true
}

fn solver(sudoku: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    sudoku.clone()
}
