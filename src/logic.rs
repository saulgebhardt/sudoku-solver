use std::collections::HashMap;

use crate::cell::SudokuCell;

/*
* This file contains the logic for the sudoku solver program.
* Currently, it only contains an is_valid function, which provides info wheter it is a legal move
* to place a value at a specific coordinate.
*/

pub struct InvalidError(Vec<Vec<SudokuCell>>);

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

pub fn backtrack_pencilmarks(
    mut sudoku: Vec<Vec<SudokuCell>>,
    pencilmarks: HashMap<(usize, usize), Vec<u8>>,
) -> Result<Vec<Vec<SudokuCell>>, InvalidError> {
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
    let possible_values = pencilmarks.get(&(row, column)).unwrap();
    for value in possible_values {
        let temp_value = value.clone();
        if is_valid(&sudoku, row, column, temp_value) {
            sudoku[row][column].value = temp_value;
            match backtrack_pencilmarks(sudoku, pencilmarks.clone()) {
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
    for value in 1..=9 as u8 {
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
