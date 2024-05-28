use prettytable::Table;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;
use sudoku_solver::backtrack;
use sudoku_solver::backtrack_pencilmarks::backtrack_pencilmarks;
use sudoku_solver::backtrack_pencilmarks::create_pencilmarks;
use sudoku_solver::cell::SudokuCell;

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
            if amount_of_puzzles == 1000 {
                break;
            }
        }
    }

    let time_to_solve = Instant::now();
    sudokus_to_solve
        .par_iter()
        .for_each(|x| solve_sudoku(x.to_owned()));
    /*for sudoku_string in sudokus_to_solve {
        solve_sudoku(sudoku_string);
    }*/
    println!("Done");
    println!(
        "Solved {} sudokus. It took {:?}.",
        amount_of_puzzles,
        time_to_solve.elapsed()
    );
}

fn solve_sudoku(sudoku_string: String) {
    let sudoku_puzzle = create_sudoku(sudoku_string);
    let pencilmarks = create_pencilmarks(&sudoku_puzzle);
    if let Ok(solution) = backtrack_pencilmarks(sudoku_puzzle, &pencilmarks) {
        print_sudoku(&solution);
    } else {
        println!("Sudoku could not be solved:");
    }
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
    /*for line in sudoku {
        for cell in line {
            print!("{}|", cell);
        }
        println!("\n");
    }
    println!("Sudoku printed");*/
}
