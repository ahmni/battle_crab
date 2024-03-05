use std::error::Error;

use crate::player::{Board, Cell, Index, Player, Position};

#[derive(Debug)]
struct GameInfo {
    board_size: usize,
    num_ships: u32,
    players: [Player; 2],
    turn: usize,
    is_running: bool,
}

pub struct Game {
    game_info: Option<GameInfo>,
}

impl Game {
    pub fn new() -> Game {
        Game { game_info: None }
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
            players: [
                Player {
                    name: player1_name.trim().to_string(),
                    board: Board(vec![vec![Cell::Empty; board_size]; board_size]),
                    valid_ship_count: num_ships,
                },
                Player {
                    name: player2_name.trim().to_string(),
                    board: Board(vec![vec![Cell::Empty; board_size]; board_size]),
                    valid_ship_count: num_ships,
                },
            ],
            turn: 0,
            board_size,
            num_ships,
            is_running: true,
        });
    }

    fn get_position_from_user(board_size: usize) -> Result<Position, Box<dyn Error>> {
        let row = loop {
            let mut raw_row = String::new();
            println!("Enter row coordinate: ");
            std::io::stdin().read_line(&mut raw_row)?;
            if let Ok(row) = Index::new(raw_row.trim().parse()?, board_size) {
                break row;
            } else {
                println!("Invalid row coordinate");
            }
        };
        let col = loop {
            let mut raw_col = String::new();
            println!("Enter col coordinate: ");
            std::io::stdin().read_line(&mut raw_col)?;
            if let Ok(col) = Index::new(raw_col.trim().parse()?, board_size) {
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
        for player in &mut game_info.players {
            println!("{} place your ships", player.name);
            for _ in 0..game_info.num_ships {
                loop {
                    let position = Self::get_position_from_user(game_info.board_size)?;
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
        println!("BattleCrab Game Begins! \n");
        while game_info.is_running {
            {
                let cur_player = &game_info.players[game_info.turn % 2];

                println!("{}'s turn", cur_player.name);
                println!("{}'s board: \n", cur_player.name);
                println!("{}", cur_player.board);
            }
            let opposite_player = &mut game_info.players[(game_info.turn + 1) % 2];

            println!("{}'s board: \n", opposite_player.name);
            opposite_player.view_board();

            // Get attack Position
            println!("Where would you like to attack?");
            let position = Self::get_position_from_user(game_info.board_size)?;

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
                let cur_player = &game_info.players[game_info.turn % 2];
                println!("{} wins!", cur_player.name);
                game_info.is_running = false;
                break;
            }

            game_info.turn += 1;
        }

        return Ok(());
    }
}
