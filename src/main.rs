use std::io;

use crate::ai::{call_openai_api, retry, ChatCompletion};

mod ai;
mod game;
mod player;
mod test;

#[tokio::main]
async fn main() {
    // Setups
    let (mut game, mut player, mut ai, mut round_id, user, assistant) = game::setup();
    let mut chat_completion = ChatCompletion::setup();

    println!("Play a game of rock, paper and scissor with Chat-GPT!");
    println!("First to 3 wins!");

    let mut game_on = true;

    // Start game
    while game_on {
        let mut message =
            format!("Round {round_id}. Please make a choice. Rock, paper or scissor?");

        chat_completion.save_msg(&user, message);

        // Get Chat-GPT choice
        let ai_choice = match call_openai_api(&chat_completion).await {
            Ok(ai_choice) => ai_choice.to_lowercase(),
            Err(_) => retry(&chat_completion).await,
        };

        ai.choose(&ai_choice).unwrap_or(String::from("rock"));

        println!("Please make your choice: ");

        // Get user choice from terminal
        'inner: loop {
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Should read the input from terminal.");

            if let Ok(c) = player.choose(&choice.trim().to_lowercase()) {
                println!("{c}");
                break 'inner;
            } else {
                println!("\nValid choices are (rock, paper or scissor).\n");
                continue 'inner;
            };
        }

        println!("Chat-GPT choosed {ai_choice}!");

        chat_completion.save_msg(&assistant, ai_choice);

        // Play the round and get the winner
        game.play(round_id, &player, &ai);

        let result = game
            .get_round_result(round_id)
            .expect("Round id should start at 1 and increment with 1.")
            .to_string();

        println!("Round {round_id} winner: {result}");

        game.update_scoreboard();

        message = game.get_comment(&result, round_id);
        println!("{message}");

        chat_completion.save_msg(&assistant, message);

        // Get Chat-GPT comment about the round
        let ai_comment = match call_openai_api(&chat_completion).await {
            Ok(ai_comment) => ai_comment,
            Err(_) => retry(&chat_completion).await,
        };
        println!("Chat-GPT: {ai_comment}\n");

        chat_completion.save_msg(&assistant, ai_comment);

        round_id += 1;

        // Play until 3 wins
        game_on = game.three_wins();
    }
}
