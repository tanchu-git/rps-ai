use crate::game::Choice;

pub struct Player {
    name: String,
    choice: Choice,
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

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn choice(&self) -> &Choice {
        &self.choice
    }

    pub fn choose(&mut self, choice: &str) -> Result<String, ()> {
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
            _ => Err(()),
        };
        result
    }
}
