#[cfg(test)]
mod test {
    use crate::ai::*;
    use crate::game::*;
    use crate::player::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn setup() -> (Game, Player, Player) {
        let game = Game::new();
        let player = Player::new("Human", "");
        let ai = Player::new("GPT-4", "");

        (game, player, ai)
    }

    #[rstest]
    #[case("rock", Choice::Rock)]
    #[case("paper", Choice::Paper)]
    #[case("scissor", Choice::Scissor)]
    fn test_player_choose(
        setup: (Game, Player, Player),
        #[case] choice: &str,
        #[case] expected: Choice,
    ) {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .install();

        let (_, mut player, _) = setup;

        player.choose(choice).unwrap();
        assert_eq!(expected, *player.choice())
    }

    #[rstest]
    #[case("rock", "paper", "GPT-4")]
    #[case("rock", "scissor", "Human")]
    #[case("rock", "rock", "Tie")]
    #[case("paper", "paper", "Tie")]
    #[case("paper", "scissor", "GPT-4")]
    #[case("paper", "rock", "Human")]
    #[case("scissor", "paper", "Human")]
    #[case("scissor", "scissor", "Tie")]
    #[case("scissor", "rock", "GPT-4")]
    fn test_game_play_integration_winner(
        setup: (Game, Player, Player),
        #[case] human_choice: &str,
        #[case] ai_choice: &str,
        #[case] winner: &str,
    ) {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .install();

        let (mut game, mut player, mut ai) = setup;
        let id = 1;

        player.choose(human_choice).unwrap();
        ai.choose(ai_choice).unwrap();

        game.play(id, &player, &ai);
        assert_eq!(winner, game.get_round_result(id).expect("id should be 1."));
    }

    #[tokio::test]
    async fn test_calling_gpt() {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .install();

        let user = String::from("user");

        let mut chat_completion = ChatCompletion::setup();

        let message = format!("Round 1. Please make a choice. Rock, paper or scissor?");

        chat_completion.save_msg(&user, message);

        match call_openai_api(&chat_completion).await {
            Ok(ai_response) => {
                dbg!(ai_response);
                assert!(true)
            }
            Err(_) => assert!(false),
        }
    }
}
