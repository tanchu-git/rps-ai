use std::io;

mod game;
mod player;
mod test;
mod ai;

fn main() {
    let (mut game, mut player, mut ai, mut round_id, mut result) = game::setup();

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
                println!("\nValid choices are (rock, paper or scissor).\n");
                continue;
            }
        };

        game.play(round_id, &player, &ai);

        result = game
            .get_round_result(round_id)
            .expect("Round id should always be greater than 0.")
            .to_string();

        println!("Round {round_id} winner: {result}\n");

        round_id += 1;

        game_on = game.three_wins();
    }
}
