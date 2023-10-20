#[derive(PartialEq, Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissor,
}

// Input a choice and output the losing choice against it
impl Choice {
    pub fn rules(&self) -> Self {
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

impl Player {
    pub fn new(name: &str, choice: &str) -> Self {
        let choice: Choice = match choice {
            "rock" => Choice::Rock,
            "paper" => Choice::Paper,
            _ => Choice::Scissor,
        };

        Self {
            name: String::from(name),
            choice,
        }
    }

    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }
    pub fn choice(&self) -> &Choice {
        &self.choice
    }

    pub fn choose(&mut self, choice: &str) -> Result<String, ()> {
        match choice {
            "rock" => {
                self.choice = Choice::Rock;
                Ok(String::from("\nYou choosed rock!"))
            }
            "paper" => {
                self.choice = Choice::Paper;
                Ok(String::from("\nYou choosed paper!"))
            }
            "scissor" => {
                self.choice = Choice::Scissor;
                Ok(String::from("\nYou choosed scissor!"))
            }
            _ => Err(()),
        }
    }
}
