use crate::is_valid;
use crate::InvalidError;
use crate::SudokuCell;
use std::collections::HashMap;

pub fn backtrack_pencilmarks(
    mut sudoku: Vec<Vec<SudokuCell>>,
    pencilmarks: &HashMap<(usize, usize), Vec<u8>>,
) -> Result<Vec<Vec<SudokuCell>>, InvalidError> {
    let mut is_filled = true;
    let mut row: usize = 0;
    let mut column: usize = 0;
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j].value == 0 {
                is_filled = false;
                row = i;
                column = j;
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
    let possible_values = pencilmarks.get(&(row, column)).unwrap();
    for value in possible_values {
        let temp_value = value.clone();
        if is_valid(&sudoku, row, column, temp_value) {
            sudoku[row][column].value = temp_value;
            match backtrack_pencilmarks(sudoku, &pencilmarks) {
                Ok(x) => return Ok(x),
                Err(InvalidError(e)) => sudoku = e,
            }
            sudoku[row][column].value = 0;
        }
    }
    Err(InvalidError(sudoku))
}

pub fn create_pencilmarks(sudoku: &Vec<Vec<SudokuCell>>) -> HashMap<(usize, usize), Vec<u8>> {
    let mut pencilmarks = HashMap::new();
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j].value == 0 {
                pencilmarks.insert((i, j), possible_values(sudoku, i, j));
            }
        }
    }
    pencilmarks
}

fn possible_values(sudoku: &Vec<Vec<SudokuCell>>, row: usize, column: usize) -> Vec<u8> {
    let mut possible: Vec<u8> = Vec::new();
    for value in 1..=9_u8 {
        let mut found = false;
        for i in 0..9 {
            if sudoku[row][i].value == value {
                found = true;
                break;
            }
            if sudoku[i][column].value == value {
                found = true;
                break;
            }
            let x_coord = (column / 3) * 3;
            let y_coord = (row / 3) * 3;
            for i in y_coord..y_coord + 3 {
                for j in x_coord..x_coord + 3 {
                    if sudoku[i][j].value == value {
                        found = true;
                        break;
                    }
                }
            }
        }
        if found == false {
            possible.push(value);
        }
    }
    possible
}
