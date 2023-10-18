use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

// AI interaction
// model - which llm to use i.e GPT-4
// messages - chat history
// temperature - ai creativity
#[derive(Debug, Serialize, Clone)]
struct ChatCompletion {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    role: String,
    content: String,
}

impl Message {
    pub fn setup() -> Vec<Self> {
        let persona = Message {
            role: String::from("system"),
            content: String::from(
                "You and I are going to play a game of rock, paper and scissor. The
                first one with 3 wins, wins the whole game.
                When you are asked to make a choice. Please provide ONLY one word 
                for an answer, so either 'ROCK', 'PAPER' or 'SCISSOR'. BUT when you
                are asked to provide commentary for how the game is going, you SHOULD
                be SNARKY and UPTIGHT with your commentary. You should be a LOUSY
                winner if you currently have more wins than me. And you should be a SORE loser if
                I currently have more wins than you.",
            ),
        };

        vec![persona]
    }

    pub fn new_msg(message: String) -> Self {
        Self {
            role: String::from("user"),
            content: message,
        }
    }
}

// Three nested structs to model AI
// response, which is a nested JSON file
#[derive(Debug, Deserialize)]
struct AIResponse {
    choices: Vec<AIChoice>,
}

#[derive(Debug, Deserialize)]
struct AIChoice {
    message: AIMessage,
}

#[derive(Debug, Deserialize)]
struct AIMessage {
    content: String,
}

// Call OpenAI API
// Map any non env errors to heap and propagate out of the function
pub async fn call_openai_api(messages: Vec<Message>) -> Result<String, Box<dyn Error + Send>> {
    dotenv().ok();

    // Extract API key & org
    let api_key = match env::var("OPEN_AI_KEY") {
        Ok(key) => key,
        Err(_) => panic!("OPEN_AI_KEY env variable NOT found!"),
    };

    let api_org = match env::var("OPEN_AI_ORG") {
        Ok(org) => org,
        Err(_) => panic!("OPEN_AI_ORG env variable NOT found!"),
    };

    // Set OpenAI API endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers for OpenAI API
    let mut headers: HeaderMap = HeaderMap::new();

    // Create API key header
    // Propagate errors out of the function for the caller to handle
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {api_key}"))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create API org header
    // Propagate errors out of the function for the caller to handle
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org)
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client to make Requests with
    // Propagate errors out of the function for the caller to handle
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion for AI to derive a response
    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.5,
    };

    // Construct a Request and a JSON body from ChatCompletion
    // Extract response from a nested JSON file - Vec -> String
    // ["choices": -> "message": -> { "content": -> String }]
    let response: AIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Return extracted response
    Ok(response.choices[0].message.content.clone())
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_calling_gpt() {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .install();

        let persona = Message {
            role: String::from("system"),
            content: String::from("You should be snarky and uptight with your answers"),
        };

        let mut ai_persona = vec![persona];

        let message = Message {
            role: "user".to_string(),
            content: "Just testing. Give me a short reply so I know it works!".to_string(),
        };

        ai_persona.push(message);

        match call_openai_api(ai_persona.clone()).await {
            Ok(ai_response) => {
                dbg!(ai_response);
                assert!(true)
            }
            Err(_) => assert!(false),
        }

        tokio::time::sleep(Duration::from_secs(3)).await;

        let msg = Message::new_msg(String::from("What time is it now?"));

        ai_persona.push(msg);

        match call_openai_api(ai_persona).await {
            Ok(ai_response) => {
                dbg!(ai_response);
                assert!(true)
            }
            Err(_) => assert!(false),
        }
    }
}
