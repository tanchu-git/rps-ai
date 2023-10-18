use crate::player::Player;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
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
