use crate::game::Game;

pub mod game;
pub mod player;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();
    game.run().unwrap();
    Ok(())
}
