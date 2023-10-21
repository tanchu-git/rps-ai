use crossterm::{
    style::{Color, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

use crate::ai::{call_openai_api, retry, ChatCompletion};

mod ai;
mod game;
mod player;

#[tokio::main]
async fn main() {
    // Setups
    let (mut game, mut player, mut ai, mut round_id, user, assistant) = game::setup();
    let mut chat_completion = ChatCompletion::setup();

    // Colour style for terminal output
    let mut stdout: std::io::Stdout = stdout();
    stdout.execute(SetForegroundColor(Color::Blue)).ok();

    println!("Play a game of rock, paper and scissor with GPT-4!");
    println!("First to 3 wins!");

    let mut game_on = true;

    // Start game
    while game_on {
        let mut message =
            format!("Round {round_id}. Please make a choice. Rock, paper or scissor?");

        // Save chat history
        chat_completion.save_msg(&user, message);

        // Get GPT-4 choice
        let ai_choice = match call_openai_api(&chat_completion).await {
            Ok(ai_choice) => ai_choice.to_lowercase(),
            Err(_) => retry(&chat_completion).await,
        };

        ai.choose(&ai_choice).unwrap_or(String::from("scissor"));

        println!("Please make your choice: ");

        // Get user choice from terminal
        'inner: loop {
            let mut choice = String::new();
            stdin()
                .read_line(&mut choice)
                .expect("Should read the input from terminal.");

            if let Ok(c) = player.choose(&choice.trim().to_lowercase()) {
                println!("{c}");
                break 'inner;
            }
            // Set warning colour
            stdout.execute(SetForegroundColor(Color::Red)).ok();
            println!("\nPlease input valid choice (rock, paper or scissor): ");
            stdout.execute(SetForegroundColor(Color::Blue)).ok();
        }

        println!("GPT-4 choosed {ai_choice}!");

        // Save chat history
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

        // Save chat history
        chat_completion.save_msg(&assistant, message);

        // Get GPT-4 comment about the round
        let ai_comment = match call_openai_api(&chat_completion).await {
            Ok(ai_comment) => ai_comment,
            Err(_) => retry(&chat_completion).await,
        };

        // Set GPT-4 colour
        stdout.execute(SetForegroundColor(Color::Yellow)).ok();
        println!("GPT-4: {ai_comment}\n");

        // Save chat history
        chat_completion.save_msg(&assistant, ai_comment);

        round_id += 1;

        // Play until 3 wins
        game_on = game.three_wins();
        stdout.execute(SetForegroundColor(Color::Blue)).ok();
    }
}
