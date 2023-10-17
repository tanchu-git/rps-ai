use crate::player::Player;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    fn rules(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissor,
            Choice::Paper => Choice::Rock,
            Choice::Scissor => Choice::Paper,
        }
    }
}

pub struct Round {
    id: u32,
    result: String,
}

impl Round {
    pub fn get_result(&self) -> &str {
        &self.result
    }

    pub fn get_round(&self) -> u32 {
        self.id
    }
}

pub struct Game {
    rounds: Vec<Round>,
}

impl Game {
    pub fn new() -> Self {
        Self { rounds: vec![] }
    }

    pub fn play_round(&self, id: u32, player: &Player, ai: &Player) -> Round {
        let (player_beats, ai_beats) = (player.choice().rules(), ai.choice().rules());

        let result = if &player_beats == ai.choice() {
            player.get_name().to_string()
        } else if &ai_beats == player.choice() {
            ai.get_name().to_string()
        } else {
            String::from("Tie")
        };

        let round = Round { id, result };

        round
    }

    pub fn save_round(&mut self, round: Round) {
        self.rounds.push(round);
    }

    pub fn three_wins(&self) -> bool {
        let mut round_iter = self.rounds.iter();

        let mut score: HashMap<&str, u8> = HashMap::new();

        // Tallying wins for each player using a hash map
        for round in round_iter.by_ref() {
            let count = score.entry(&round.result).or_insert(0);
            *count += 1;
        }

        println!("{score:?}");

        // Check for 3 wins
        for (key, value) in score.iter() {
            let b = match value {
                3 if *key == "Human" || *key == "Chat-GPT" => false,
                _ => continue,
            };

            return b;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_winner_and_integration() {
        let game = Game::new();

        let winning_choices = vec!["rock", "paper", "scissor"];
        let losing_choices = vec!["scissor", "rock", "paper"];

        let mut player = Player::new("Human", "rock");
        let mut ai = Player::new("Chat-GPT", "rock");
        let mut round = game.play_round(1, &player, &ai);
        assert_eq!("Tie", round.get_result());

        let mut losing_iter = losing_choices.iter();
        for choice in winning_choices {
            player.choose(choice).unwrap();
            ai.choose(losing_iter.next().unwrap()).unwrap();
            round = game.play_round(2, &player, &ai);
            assert_eq!("Human", round.get_result());
        }
    }
}
