use std::io::{self, Write};

fn main() {
    //TODO: Read a single sudoku from a file, and parse it as a 2d array(Vec) of u8
    let temp_string = String::from(
        "050703060007000800000816000000030000005000100730040086906000204840572093000409000",
    );
    let sudoku_puzzle = create_sudoku(temp_string);
    print(&sudoku_puzzle);
}

fn create_sudoku(input_string: String) -> Vec<Vec<u8>> {
    let mut input_iterator = input_string.chars().into_iter();
    let mut sudoku_puzzle: Vec<Vec<u8>> = Vec::new();
    for _ in 0..9 {
        let mut line: Vec<u8> = Vec::new();
        for _ in 0..9 {
            let temp_u8 = input_iterator.next().unwrap() as u8 - 48;
            line.push(temp_u8);
        }
        sudoku_puzzle.push(line);
    }
    sudoku_puzzle
}

fn print(sudoku: &Vec<Vec<u8>>) {
    for line in sudoku {
        for row in line {
            print!("{}", row);
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}

//TODO: Implement a sudoku solving algorithm. Backtracking seems like a decent start
//
//
//TODO: Print sudoku to console cleanly, think if we need the bars, what values look like, et
//cetera.
//
//
//TODO: Implement some small timing tool, useful for assessing efficiency of algorithm and show
//paralellism.
