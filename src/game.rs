use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

struct Player {
    name: String,
    choice: Choice,
}

struct Game {
    rounds: u8,
    round: Vec<Round>,
}

struct Round {
    id: u8,
    players: HashMap<String, Choice>,
    winner: String,
}

impl Player {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn choice(&mut self, choice: &str) {
        match choice {
            "rock" => self.choice = Choice::Rock,
            "paper" => self.choice = Choice::Paper,
            _ => self.choice = Choice::Scissor,
        }
    }
}

impl Round {
    pub fn check_winner(round: u8, player: Player, ai: Player) {
        let mut players = HashMap::new();
        let mut winner = String::new(); 

        if player.choice == ai.choice {
            winner = String::from("Tie");
        }
        match player.choice {
            Choice::Rock => {
                if ai.choice == Choice::Paper {
                    winner = String::from("AI");
                } else {
                    winner = String::from("Player");
                }
            },
            Choice::Paper => {
                if ai.choice == Choice::Scissor {
                    winner = String::from("AI");
                } else {
                    winner = String::from("Player");
                }
            },
            Choice::Scissor => {
                if ai.choice == Choice::Rock {
                    winner = String::from("AI");
                } else {
                    winner = String::from("Player");
                }
            }
        };    

        players.insert(player.name, player.choice);
        players.insert(ai.name, ai.choice);

        let round = Round {
            id: round,
            players,
            winner,
        };
    }
}