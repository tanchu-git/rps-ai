#[derive(PartialEq)]
enum Choice {
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

struct Player {
    name: String,
    choice: Choice,
}

struct Game {
    total: u8,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
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

    pub fn choice(&mut self, choice: &str) {
        match choice {
            "rock" => self.choice = Choice::Rock,
            "paper" => self.choice = Choice::Paper,
            _ => self.choice = Choice::Scissor,
        }
    }
}

impl Game {
    pub fn play_round(&self, round: u8, player: Player, ai: Player) -> Round {
        let (player_beats, ai_beats) = (player.choice.rules(), ai.choice.rules());

        let result = if player_beats == ai.choice {
            player.name
        } else if ai_beats == player.choice {
            ai.name
        } else {
            String::from("Tie")
        };

        let round = Round { id: round, result };

        round
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_winner() {
        let game = Game {
            total: 5,
            rounds: vec![],
        };

        let mut player = Player::new("Yonda", "rock");
        let mut ai = Player::new("Chat-GPT", "rock");
        let mut round = game.play_round(1, player, ai);
        assert_eq!("Tie", round.result);

        player = Player::new("Yonda", "rock");
        ai = Player::new("Chat-GPT", "scissor");
        round = game.play_round(1, player, ai);
        assert_eq!("Yonda", round.result);

        player = Player::new("Yonda", "rock");
        ai = Player::new("Chat-GPT", "paper");
        round = game.play_round(1, player, ai);
        assert_eq!("Chat-GPT", round.result);

        player = Player::new("Yonda", "scissor");
        ai = Player::new("Chat-GPT", "rock");
        round = game.play_round(1, player, ai);
        assert_eq!("Chat-GPT", round.result);

        player = Player::new("Yonda", "scissor");
        ai = Player::new("Chat-GPT", "paper");
        round = game.play_round(1, player, ai);
        assert_eq!("Yonda", round.result);

        player = Player::new("Yonda", "paper");
        ai = Player::new("Chat-GPT", "scissor");
        round = game.play_round(1, player, ai);
        assert_eq!("Chat-GPT", round.result);

        player = Player::new("Yonda", "paper");
        ai = Player::new("Chat-GPT", "rock");
        round = game.play_round(1, player, ai);
        assert_eq!("Yonda", round.result);
    }
}
