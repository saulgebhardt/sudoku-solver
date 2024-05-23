use std::fmt;

#[derive(Debug)]
pub struct SudokuCell {
    pub value: u8,
    pub movable: bool,
}

impl SudokuCell {
    pub fn new(x: char) -> SudokuCell {
        let mut movable = false;
        let value = x as u8 - 48;
        if value == 0 {
            movable = true;
        }
        SudokuCell { value, movable }
    }
}

impl fmt::Display for SudokuCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
