mod cell;
mod logic;

use cell::SudokuCell;
use logic::backtrack;
use prettytable::Table;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

//NOTE: We need to store the values of the Cells in structs so we can keep track whether we are
//allowed to change values. Based on if the value was there from the start, we can also assign a
//color to it.
//
//NOTE: Can we confirm somehow that there exists only 1 solution? If there exist more than 1, the
//sudoku is invalid.
//

fn main() {
    let file = File::open("easy.txt").expect("File not found!");
    let reader = BufReader::new(file);
    let mut sudokus_to_solve: Vec<String> = Vec::new();
    let mut amount_of_puzzles = 0;
    for line in reader.lines() {
        if let Ok(x) = line {
            sudokus_to_solve.push(String::from(x.split(' ').collect::<Vec<&str>>()[1]));
            amount_of_puzzles += 1;
        }
    }

    let time_to_solve = Instant::now();
    for sudoku_string in sudokus_to_solve {
        let sudoku_puzzle = create_sudoku(sudoku_string);
        if let Ok(solution) = backtrack(sudoku_puzzle) {
            print_sudoku(&solution);
        } else {
            println!("Sudoku could not be solved:");
        }
    }
    println!("Done");
    println!(
        "Solved {} sudokus. It took {:?}.",
        amount_of_puzzles,
        time_to_solve.elapsed()
    );
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

fn print_sudoku(sudoku: &Vec<Vec<SudokuCell>>) {
    let mut table = Table::new();
    for line in sudoku {
        table.add_row(line.into());
    }
    table.printstd();
}
//TODO: Implement some small timing tool, useful for assessing efficiency of algorithm and show
//paralellism.
