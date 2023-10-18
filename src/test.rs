#[cfg(test)]
mod test {
    use crate::game::*;
    use crate::player::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn setup() -> (Game, Player, Player) {
        let game = Game::new();
        let player = Player::new("Human", "");
        let ai = Player::new("Chat-GPT", "");

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
        let (_, mut player, _) = setup;

        player.choose(choice).unwrap();
        assert_eq!(expected, *player.choice())
    }

    #[rstest]
    #[case("rock", "paper", "Chat-GPT")]
    #[case("rock", "scissor", "Human")]
    #[case("rock", "rock", "Tie")]
    #[case("paper", "paper", "Tie")]
    #[case("paper", "scissor", "Chat-GPT")]
    #[case("paper", "rock", "Human")]
    #[case("scissor", "paper", "Human")]
    #[case("scissor", "scissor", "Tie")]
    #[case("scissor", "rock", "Chat-GPT")]
    fn test_game_play_integration_winner(
        setup: (Game, Player, Player),
        #[case] human_choice: &str,
        #[case] ai_choice: &str,
        #[case] winner: &str,
    ) {
        let (mut game, mut player, mut ai) = setup;

        player.choose(human_choice).unwrap();
        ai.choose(ai_choice).unwrap();

        game.play(1, &player, &ai);
        assert_eq!(
            winner,
            game.get_round_result(1).expect("Should be greater than 0.")
        );
    }
}
