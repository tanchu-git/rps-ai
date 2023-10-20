use crate::player::Player;
use std::collections::HashMap;

struct Round {
    _id: usize,
    result: String,
}

impl Round {
    fn get_result(&self) -> String {
        String::from(&self.result)
    }
}

pub struct Game {
    rounds: Vec<Round>,
    scoreboard: HashMap<String, u32>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            rounds: vec![],
            scoreboard: HashMap::new(),
        }
    }

    // Using Choice::rules() to get the winner
    pub fn play(&mut self, id: usize, player: &Player, ai: &Player) {
        let (player_beats, ai_beats) = (player.choice().rules(), ai.choice().rules());

        let result = if &player_beats == ai.choice() {
            player.get_name()
        } else if &ai_beats == player.choice() {
            ai.get_name()
        } else {
            String::from("Tie")
        };

        let round = Round { _id: id, result };

        self.rounds.push(round);
    }

    pub fn get_round_result(&self, id: usize) -> Option<&str> {
        let id = match id {
            0 => id,
            _ => id - 1,
        };

        match self.rounds.get(id) {
            Some(round) => Some(&round.result),
            None => None,
        }
    }

    pub fn update_scoreboard(&mut self) {
        let round_iter = self.rounds.iter();

        let mut score: HashMap<String, u32> = HashMap::new();

        // Tallying wins for each player using a hash map
        for round in round_iter.as_ref() {
            let count = score.entry(round.get_result()).or_insert(0);
            *count += 1;
        }

        self.scoreboard = score;

        println!("Scoreboard: {:?}\n", self.scoreboard);
    }

    // Check for 3 wins for each player
    pub fn three_wins(&self) -> bool {
        for (key, value) in &self.scoreboard {
            let b = match value {
                3 if *key == "Human" || *key == "Chat-GPT" => false,
                _ => continue,
            };

            return b;
        }
        true
    }

    // Ask Chat-GPT for commentary about the state of the game
    pub fn get_comment(&self, result: &String, round_id: usize) -> String {
        match &result[..] {
            "Human" => {
                if self.three_wins() {
                    format!("Human won round {round_id}. Please make a comment.")
                } else {
                    String::from(
                        "Human have got 3 wins, human won the whole game! Please make a comment.",
                    )
                }
            }
            "Chat-GPT" => {
                if self.three_wins() {
                    format!("You the AI won round {round_id}. Please make a comment.")
                } else {
                    String::from(
                        "You the AI have got 3 wins, you won the whole game! Please make a comment.")
                }
            }
            _ => format!("We tied round {round_id}. Please make a comment."),
        }
    }
}

pub fn setup() -> (Game, Player, Player, usize, String, String) {
    let game = Game::new();
    let player = Player::new("Human", "");
    let ai = Player::new("Chat-GPT", "");
    let round_id: usize = 1;
    let user = String::from("user");
    let assistant = String::from("assistant");

    (game, player, ai, round_id, user, assistant)
}
