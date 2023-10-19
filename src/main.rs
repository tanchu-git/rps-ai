use std::io;
use std::time::Duration;

use crate::ai::{call_openai_api, ChatCompletion};

mod ai;
mod game;
mod player;
mod test;

#[tokio::main]
async fn main() {
    // Setups
    let (mut game, mut player, mut ai, mut round_id, mut result) = game::setup();
    let mut chat_completion = ChatCompletion::setup();

    println!("Play a game of rock, paper and scissor with Chat-GPT!");
    println!("First to 3 wins!");

    let mut game_on = true;

    // Play until 3 wins
    while game_on {
        // Chat-GPT
        let mut message =
            format!("Round {round_id}. Please make a choice. Rock, paper or scissor?");

        chat_completion.save_msg(message);

        // Get Chat-GPT choice
        let ai_choice = match call_openai_api(&chat_completion).await {
            Ok(ai_choice) => ai_choice.to_lowercase(),
            Err(_) => call_openai_api(&chat_completion)
                .await
                .expect("Failed twice to call OpenAI"),
        };

        ai.choose(&ai_choice).unwrap_or(String::from("rock"));

        // User
        println!("Please make your choice: ");

        let mut choice = String::new();

        // Get user choice from terminal
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        if let Ok(c) = player.choose(&choice.trim().to_lowercase()) {
            println!("{c}");
        } else {
            println!("\nValid choices are (rock, paper or scissor).\n");
            continue;
        };

        println!("Chat-GPT choosed {ai_choice}!");

        // Game
        // Play the round and get the winner
        game.play(round_id, &player, &ai);

        result = game
            .get_round_result(round_id)
            .expect("Round id should start at 1 and increment with 1.")
            .to_string();

        println!("Round {round_id} winner: {result}\n");

        // Chat-GPT
        // Get Chat-GPT comment about the round
        message = match &result[..] {
            "Human" => if game.three_wins() {
                format!("I won round {round_id}. Please make a comment.")
            } else {
                "I have got 3 wins, I won the whole game! Please make a comment.".to_string()
            }
            "Chat-GPT" => if game.three_wins() {
                format!("You won round {round_id}. Please make a comment.")
            } else {
                "You have got 3 wins, You won the whole game! Please make a comment.".to_string()
            }
            _ => format!("We tied round {round_id}. Please make a comment."),
        };

        chat_completion.save_msg(message);

        let ai_comment = match call_openai_api(&chat_completion).await {
            Ok(ai_choice) => ai_choice,
            Err(_) => call_openai_api(&chat_completion)
                .await
                .expect("Failed twice to call OpenAI"),
        };

        // Give Chat-GPT some time to make a commentary
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Game
        println!("Chat-GPT: {ai_comment}\n");

        // Next round -> next loop
        round_id += 1;

        game_on = game.three_wins();
    }
}
