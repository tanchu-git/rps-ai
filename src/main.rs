use crate::ai::{call_openai_api, ChatMessage};
use std::io;

mod ai;
mod game;
mod player;
mod test;

#[tokio::main]
async fn main() {
    let (mut game, mut player, mut ai, mut round_id, mut result) = game::setup();
    let mut ai_persona = ChatMessage::setup();

    println!("Play a game of rock, paper and scissor with Chat-GPT!");
    println!("First to 3 wins!");

    let mut game_on = true;

    while game_on {
        let message = ChatMessage::new_msg(format!(
            "Round {round_id}. Please make a choice. Rock, paper or scissor?"
        ));

        ai_persona.push(message);

        let ai_choice = match call_openai_api(&ai_persona).await {
            Ok(ai_choice) => ai_choice,
            Err(_) => call_openai_api(&ai_persona)
                .await
                .expect("Failed twice to call OpenAI"),
        };

        ai.choose(&ai_choice).unwrap(); //todo!

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
            .expect("Round id should start at 1 and increment with 1.")
            .to_string();

        println!("Round {round_id} winner: {result}\n");

        round_id += 1;

        game_on = game.three_wins();
    }
}
