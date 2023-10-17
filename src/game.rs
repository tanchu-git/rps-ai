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

pub struct Player {
    name: String,
    choice: Choice,
}

#[derive(Debug)]
pub struct Game {
    total: u8,
    rounds: Vec<Round>,
}

#[derive(Debug)]
pub struct Round {
    id: u8,
    result: String,
}

impl Player {
    pub fn new(name: &str, choice: &str) -> Self {
        let choice: Choice = match choice {
            "rock" => Choice::Rock,
            "paper" => Choice::Paper,
            _ => Choice::Scissor,
        };

        Self {
            name: name.to_string(),
            choice,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn choice(&mut self, choice: &str) -> Result<String, ()>{
        let result = match choice {
            "rock" => {
                self.choice = Choice::Rock;
                Ok(String::from("You choosed rock!"))
            }
            "paper" => {
                self.choice = Choice::Paper;
                Ok(String::from("You choosed paper!"))
            }
            "scissor" => {
                self.choice = Choice::Scissor;
                Ok(String::from("You choosed scissor!"))
            }
            _ => Err(())
        };

        result
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            total: 5,
            rounds: vec![],
        }
    }

    pub fn play_round(&self, round: u8, player: &Player, ai: &Player) -> Round {
        let (player_beats, ai_beats) = (player.choice.rules(), ai.choice.rules());

        let result = if player_beats == ai.choice {
            player.name.clone()
        } else if ai_beats == player.choice {
            ai.name.clone()
        } else {
            String::from("Tie")
        };

        let round = Round { id: round, result };

        round
    }
}

impl Round {
    pub fn get_result(&self) -> &str {
        self.result.as_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_winner() {
        let game = Game::new();

        let mut player = Player::new("Yonda", "rock");
        let mut ai = Player::new("Chat-GPT", "rock");
        let mut round = game.play_round(1, &player, &ai);
        assert_eq!("Tie", round.get_result());

        player = Player::new("Yonda", "rock");
        ai = Player::new("Chat-GPT", "scissor");
        round = game.play_round(2, &player, &ai);
        assert_eq!("Yonda", round.get_result());

        player = Player::new("Yonda", "rock");
        ai = Player::new("Chat-GPT", "paper");
        round = game.play_round(1, &player, &ai);
        assert_eq!("Chat-GPT", round.get_result());

        player = Player::new("Yonda", "scissor");
        ai = Player::new("Chat-GPT", "rock");
        round = game.play_round(1, &player, &ai);
        assert_eq!("Chat-GPT", round.get_result());

        player = Player::new("Yonda", "scissor");
        ai = Player::new("Chat-GPT", "paper");
        round = game.play_round(1, &player, &ai);
        assert_eq!("Yonda", round.get_result());

        player = Player::new("Yonda", "paper");
        ai = Player::new("Chat-GPT", "scissor");
        round = game.play_round(1, &player, &ai);
        assert_eq!("Chat-GPT", round.get_result());

        player = Player::new("Yonda", "paper");
        ai = Player::new("Chat-GPT", "rock");
        round = game.play_round(1, &player, &ai);
        assert_eq!("Yonda", round.get_result());
    }
}
