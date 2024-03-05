use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub board: Board,
    pub valid_ship_count: u32,
}

impl Player {
    pub fn edit_board(&mut self, position: &Position, cell: Cell) -> Result<(), Box<dyn Error>> {
        let row: usize = position.0.clone().into();
        let col: usize = position.1.clone().into();
        if let Cell::Ship = cell {
            if let Cell::Ship = self.board.0[row][col] {
                Err("Ship already placed")?;
            }
        }
        self.board.0[row][col] = cell;
        Ok(())
    }

    pub fn view_board(&self) {
        for row in &self.board.0 {
            for cell in row {
                if let Cell::Ship = cell {
                    print!("O");
                } else {
                    print!("{}", cell);
                }
            }
            print!("\n");
        }

        print!("\n");
    }

    pub fn is_hit(&self, position: &Position) -> bool {
        let row: usize = position.0.clone().into();
        let col: usize = position.1.clone().into();
        if let Cell::Ship = self.board.0[row][col] {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Ship,
    Hit,
    Miss,
}

#[derive(Debug)]
pub struct Index(usize);

impl Index {
    pub fn new(index: usize, board_size: usize) -> Result<Index, Box<dyn Error>> {
        if index < board_size {
            Ok(Index(index))
        } else {
            Err("Index out of bounds")?
        }
    }
}

impl From<Index> for usize {
    fn from(index: Index) -> usize {
        index.0
    }
}

impl Clone for Index {
    fn clone(&self) -> Index {
        Index(self.0)
    }
}

#[derive(Debug)]
pub struct Position(pub Index, pub Index);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "O"),
            Cell::Ship => write!(f, "S"),
            Cell::Hit => write!(f, "H"),
            Cell::Miss => write!(f, "M"),
        }
    }
}

#[derive(Debug)]
pub struct Board(pub Vec<Vec<Cell>>);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
