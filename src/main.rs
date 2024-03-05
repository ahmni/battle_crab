pub mod game {
    use std::{error::Error, fmt, str::FromStr};

    #[derive(Debug)]
    struct Player {
        name: String,
        board: Board,
        valid_ship_count: u32,
    }

    impl Player {
        fn edit_board(&mut self, position: &Position, cell: Cell) -> Result<(), Box<dyn Error>> {
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

        fn view_board(&self) {
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

        fn is_hit(&self, position: &Position) -> bool {
            let row: usize = position.0.clone().into();
            let col: usize = position.1.clone().into();
            if let Cell::Ship = self.board.0[row][col] {
                return true;
            }
            false
        }
    }

    #[derive(Debug)]
    struct GameInfo {
        player1: Player,
        player2: Player,
        is_player1_turn: bool,
        num_ships: u32,
        is_running: bool,
    }

    #[derive(Debug, Clone)]
    enum Cell {
        Empty,
        Ship,
        Hit,
        Miss,
    }

    #[derive(Debug)]
    struct Row(usize);

    impl Row {
        pub fn new(row: usize, board_size: usize) -> Result<Row, Box<dyn Error>> {
            if row < board_size {
                Ok(Row(row))
            } else {
                Err("Row out of bounds")?
            }
        }
    }

    impl FromStr for Row {
        type Err = Box<dyn Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let row: usize = s.trim().parse()?;
            Row::new(row, 10)
        }
    }

    impl From<Row> for usize {
        fn from(row: Row) -> usize {
            row.0
        }
    }

    impl Clone for Row {
        fn clone(&self) -> Row {
            Row(self.0)
        }
    }

    #[derive(Debug)]
    struct Col(usize);

    impl Col {
        pub fn new(col: usize, board_size: usize) -> Result<Col, Box<dyn Error>> {
            if col < board_size {
                Ok(Col(col))
            } else {
                Err("Row out of bounds")?
            }
        }
    }

    impl FromStr for Col {
        type Err = std::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let col: usize = s.trim().parse()?;
            Ok(Col(col))
        }
    }

    impl Clone for Col {
        fn clone(&self) -> Col {
            Col(self.0)
        }
    }

    impl From<Col> for usize {
        fn from(col: Col) -> usize {
            col.0
        }
    }

    #[derive(Debug)]
    struct Position(Row, Col);

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
    struct Board(Vec<Vec<Cell>>);

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

    pub struct Game {
        game_info: Option<GameInfo>,
    }

    impl Game {
        pub fn new() -> Result<Game, Box<dyn Error>> {
            Ok(Game { game_info: None })
        }

        fn get_game_info_from_user(&mut self) -> Result<GameInfo, Box<dyn Error>> {
            let mut player1_name = String::new();
            let mut player2_name = String::new();
            println!("Enter player 1 name: ");
            std::io::stdin().read_line(&mut player1_name)?;
            println!("Enter player 2 name: ");
            std::io::stdin().read_line(&mut player2_name)?;
            let num_ships = loop {
                let mut raw_num_ships = String::new();
                println!("Enter number of ships: ");
                std::io::stdin().read_line(&mut raw_num_ships)?;
                if let Ok(num_ships) = raw_num_ships.trim().parse() {
                    break num_ships;
                } else {
                    println!("Invalid number of ships");
                }
            };
            let board_size = loop {
                let mut raw_board_size = String::new();
                println!("Enter board size: ");
                std::io::stdin().read_line(&mut raw_board_size)?;
                if let Ok(board_size) = raw_board_size.trim().parse() {
                    break board_size;
                } else {
                    println!("Invalid board size");
                }
            };

            return Ok(GameInfo {
                player1: Player {
                    name: player1_name.trim().to_string(),
                    board: Board(vec![vec![Cell::Empty; board_size]; board_size]),
                    valid_ship_count: num_ships,
                },
                player2: Player {
                    name: player2_name.trim().to_string(),
                    board: Board(vec![vec![Cell::Empty; board_size]; board_size]),
                    valid_ship_count: num_ships,
                },
                is_player1_turn: true,
                num_ships,
                is_running: true,
            });
        }

        fn get_position_from_user(board_size: usize) -> Result<Position, Box<dyn Error>> {
            let row = loop {
                let mut raw_row = String::new();
                println!("Enter row coordinate: ");
                std::io::stdin().read_line(&mut raw_row)?;
                if let Ok(row) = Row::new(raw_row.trim().parse()?, board_size) {
                    break row;
                } else {
                    println!("Invalid row coordinate");
                }
            };
            let col = loop {
                let mut raw_col = String::new();
                println!("Enter col coordinate: ");
                std::io::stdin().read_line(&mut raw_col)?;
                if let Ok(col) = Col::new(raw_col.trim().parse()?, board_size) {
                    break col;
                } else {
                    println!("Invalid col coordinate");
                }
            };

            Ok(Position(row, col))
        }

        fn init(&mut self) -> Result<(), Box<dyn Error>> {
            self.game_info = Some(self.get_game_info_from_user()?);

            // Player 1 and 2 place ships
            let game_info = self.game_info.as_mut().unwrap();
            let board_size = game_info.player1.board.0.len();
            for player in vec![&mut game_info.player1, &mut game_info.player2] {
                println!("{} place your ships", player.name);
                for _ in 0..game_info.num_ships {
                    loop {
                        // todo: figure out how to refactor so that I dont have to
                        // pass board_size to get_position_from_user
                        let position = Self::get_position_from_user(board_size)?;
                        match player.edit_board(&position, Cell::Ship) {
                            Ok(_) => {
                                println!("Ship placed");
                                println!("Current board: \n{}", player.board);
                                break;
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                }
            }

            Ok(())
        }

        pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
            self.init()?;
            let game_info = self.game_info.as_mut().unwrap();
            while game_info.is_running {
                println!("BattleCrab Game Begins! \n");
                let cur_player;
                let opposite_player;
                if game_info.is_player1_turn {
                    cur_player = &mut game_info.player1;
                    opposite_player = &mut game_info.player2;
                } else {
                    cur_player = &mut game_info.player2;
                    opposite_player = &mut game_info.player1;
                }
                println!("{}'s turn", cur_player.name);
                println!("{}'s board: \n", cur_player.name);
                println!("{}", cur_player.board);
                println!("{}'s board: \n", opposite_player.name);
                opposite_player.view_board();

                // Get attack Position
                println!("Where would you like to attack?");
                let position = Self::get_position_from_user(cur_player.board.0.len())?;
                // Check if attack is a hit or Miss
                let is_hit = opposite_player.is_hit(&position);
                // Update board
                if is_hit {
                    println!("Hit!");
                    opposite_player.edit_board(&position, Cell::Hit)?;
                    opposite_player.valid_ship_count -= 1;
                } else {
                    println!("Miss!");
                    opposite_player.edit_board(&position, Cell::Miss)?;
                }
                println!("{}'s board: \n", opposite_player.name);
                opposite_player.view_board();
                // Check if winer
                if opposite_player.valid_ship_count == 0 {
                    println!("{} wins!", cur_player.name);
                    game_info.is_running = false;
                    break;
                }
                game_info.is_player1_turn = !game_info.is_player1_turn;

                let mut cmd = String::new();
                println!("Enter a command: ");
                std::io::stdin().read_line(&mut cmd)?;
                let cmd = cmd.trim();

                match cmd {
                    "quit" | "q" => {
                        self.game_info.as_mut().unwrap().is_running = false;
                        break;
                    }
                    "info" | "i" => {
                        println!("Game info: {:?}", game_info);
                    }
                    "board" | "b" => {
                        println!("Player 1 board: \n{}", game_info.player1.board);
                        println!("Player 2 board: \n{}", game_info.player2.board);
                    }
                    _ => {
                        println!("Invalid command");
                    }
                }
            }

            return Ok(());
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = game::Game::new().unwrap();
    game.run().unwrap();
    Ok(())
}
