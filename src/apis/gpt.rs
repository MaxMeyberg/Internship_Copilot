use open_ai::{OpenAI, ClientOptions};
use open_ai::resources::chat::{
    ChatCompletionContent::Text,
    ChatCompletionCreateParams,
    ChatCompletionMessageParam::{System, User},
};
use::dotenv::dotenv;
use anyhow::{Result, Context};
use std::path::Path;
use tokio::fs;

pub async fn generate_from_gpt(txt_file: &str, json_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    
    let system_prompt = fs::read_to_string(Path::new(txt_file))
        .await
        .context("Failed to read system prompt file")?;
    
    dotenv().ok(); // load up .env file, same as "load_dotenv()" in python
    // Get the API key and turn it into a string
    let api_key: String = std::env::var("OPEN_AI_API_KEY").expect("Missing OPEN AI API key, is it gone?"); 
    let mut options = ClientOptions::new();
    options.api_key = Some(api_key);
    let openai = OpenAI::new(options)?;

   
    let completion = openai.chat.completions.create(ChatCompletionCreateParams {
        messages: vec![
            System{ content: &system_prompt , name: None },
            User{ content: Text(json_str), name: None },
        ],
        model: "gpt-4o-mini",
        ..Default::default()
    }).await?;



    let message = completion
    .choices
    .first()
    .and_then(|choice| choice.message.content.as_ref())
    .ok_or("No response generated")?
    .clone();

    Ok(message)
}