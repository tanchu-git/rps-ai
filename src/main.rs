use crate::player::Player;
use game::Game;
use std::io;

mod game;
mod player;
mod test;

fn main() {
    let mut game = Game::new();
    let mut player = Player::new("Human", "");
    let mut ai = Player::new("Chat-GPT", "paper");
    let mut round_id = 1;

    println!("Play a game of rock, paper and scissor with Chat-GPT!");
    println!("First to 3 wins!");

    let mut game_on = true;

    while game_on {
        println!("Please make your choice: ");

        let mut choice = String::new();

        // Take user input from terminal
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match player.choose(&choice.trim().to_lowercase()) {
            Ok(c) => {
                println!("{c}");
            }
            Err(()) => {
                println!("\nPlease make a valid choice (rock, paper or scissor).\n");
                continue;
            }
        };

        let round = game.play_round(round_id, &player, &ai);

        println!("Round {round_id} winner: {}", round.get_result());

        game.save_round(round);

        round_id += 1;

        game_on = game.three_wins();
    }
}
