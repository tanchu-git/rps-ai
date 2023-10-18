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
    _id: usize,
    result: String,
}

impl Round {
    fn get_result(&self) -> &str {
        &self.result
    }
}

pub struct Game {
    rounds: Vec<Round>,
}

impl Game {
    pub fn new() -> Self {
        Self { rounds: vec![] }
    }

    pub fn play(&mut self, id: usize, player: &Player, ai: &Player) {
        let (player_beats, ai_beats) = (player.choice().rules(), ai.choice().rules());

        let result = if &player_beats == ai.choice() {
            player.get_name().to_string()
        } else if &ai_beats == player.choice() {
            ai.get_name().to_string()
        } else {
            String::from("Tie")
        };

        let round = Round { _id: id, result };

        self.rounds.push(round);
    }

    pub fn get_round_result(&self, id: usize) -> Option<&str> {
        match self.rounds.get(id - 1) {
            Some(result) => Some(result.get_result()),
            None => None,
        }
    }

    pub fn three_wins(&self) -> bool {
        let mut round_iter = self.rounds.iter();

        let mut score: HashMap<&str, u8> = HashMap::new();

        // Tallying wins for each player using a hash map
        for round in round_iter.by_ref() {
            let count = score.entry(&round.result).or_insert(0);
            *count += 1;
        }

        println!("Scoreboard:");
        println!("{score:?}\n");

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

pub fn setup() -> (Game, Player, Player, usize, String) {
    let game = Game::new();
    let player = Player::new("Human", "");
    let ai = Player::new("Chat-GPT", "paper");
    let round_id: usize = 1;
    let result = String::new();

    (game, player, ai, round_id, result)
}
