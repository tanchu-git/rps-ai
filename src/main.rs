use game::{Game, Player};
use std::io;

mod game;

fn main() {
    let game = Game::new();
    let mut player = Player::new("Player", "");
    let mut ai = Player::new("AI", "paper");

    println!("Play a game of rock, paper and scissor with Chat-GPT!");
    println!("Best of five!");
    // println!("Please choose your name: ");        

    // let mut name = String::new();

    // // Take user input from terminal
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read input");

    // player.set_name(name.trim());

    loop {      
        println!("Please make your choice: ");

        let mut choice = String::new();

        // Take user input from terminal
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match player.choice(&choice.trim().to_lowercase()) {
            Ok(c) => {
                println!("{c}");
                c
            }
            Err(()) => {
                println!("\nPlease make a valid choice (rock, paper or scissor).\n");
                continue;
            }
        };

        let round = game.play_round(1, &player, &ai);

        println!("Winner: {}", round.get_result());
    }  
}
