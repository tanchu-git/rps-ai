use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{env, process};

// model - which llm to use i.e GPT-4
// messages - interaction history
// temperature - ai response creativity
#[derive(Debug, Serialize)]
pub struct ChatCompletion {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

impl ChatCompletion {
    pub fn setup() -> Self {
        Self {
            model: String::from("gpt-4"),
            messages: ChatMessage::setup(),
            temperature: 0.2,
        }
    }

    // Save interaction history
    pub fn save_msg(&mut self, role: &String, content: String) {
        self.messages.push(ChatMessage::save(role, content));
    }
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

impl ChatMessage {
    fn setup() -> Vec<Self> {
        // Set AI persona and instructions
        let persona = ChatMessage {
            role: String::from("system"),
            content: String::from(
                "You the AI and a human are going to play a game of rock, paper and scissor.
                The first one with 3 wins, wins the whole game.
                When you are asked to make a choice. Please provide ONLY one word
                for an answer, so either 'ROCK', 'PAPER' or 'SCISSOR'. BUT when you
                are asked to provide commentary for how the game is going, you SHOULD
                be SNARKY and UPTIGHT with your commentary. You should be a LOUSY
                winner if you currently have more wins than the human. And you should
                be a SORE loser if human currently have more wins than you.",
            ),
        };

        vec![persona]
    }

    fn save(role: &String, content: String) -> Self {
        Self {
            role: String::from(role),
            content,
        }
    }
}

// Three nested structs to model GPT-4 response from API
// Nested JSON file - ["choices": "message": {"content": String}]
#[derive(Debug, Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

type APIError = Box<dyn std::error::Error + Send>;

// Call OpenAI API
// Map any non env errors to Box<T> and propagate out of the function
pub async fn call_openai_api(chat_completion: &ChatCompletion) -> Result<String, APIError> {
    dotenv().ok();

    // Extract API key & org
    // Exit app if error
    let Ok(api_key) = env::var("OPEN_AI_KEY") else {
        eprintln!("OPEN_AI_KEY env variable NOT found!");
        process::exit(256);
    };

    let Ok(api_org) = env::var("OPEN_AI_ORG") else {
        eprintln!("OPEN_AI_ORG env variable NOT found!");
        process::exit(256);
    };

    // Set OpenAI API endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers for OpenAI API
    let mut headers: HeaderMap = HeaderMap::new();

    // Create API key and org header and return error to caller
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {api_key}"))
            .map_err(|e| -> APIError { Box::new(e) })?,
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org).map_err(|e| -> APIError { Box::new(e) })?,
    );

    // Create client to make Requests with and return error to caller
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> APIError { Box::new(e) })?;

    // Build a Request and create a JSON body from ChatCompletion
    // Deserialize response as JSON body and return any error to caller
    let response: Response = client
        .post(url)
        .json(chat_completion)
        .send()
        .await
        .map_err(|e| -> APIError { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> APIError { Box::new(e) })?;

    // Return extracted response
    // ["choices": -> "message": -> {"content": -> String}]
    Ok(response.choices[0].message.content.clone())
}

// Retry calling API endpoint
// Exit app with error message if retry failed
pub async fn retry(chat_completion: &ChatCompletion) -> String {
    call_openai_api(chat_completion)
        .await
        .unwrap_or_else(|err| {
            eprintln!("API error: {err}");
            process::exit(256);
        })
}
