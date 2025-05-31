use reqwest;
use serde_json::Value;
use std::error::Error;
use::dotenv::dotenv;

//TODO: Cycle through the API keys automatically
pub async fn get_email_from_linkedin(linkedin_url: &str) -> Result<String, Box<dyn Error>> {
    // URL encode the LinkedIn URL
    let encoded_url = urlencoding::encode(linkedin_url);
    
    // Build the API URL
    let url = format!(
        "https://api.apollo.io/api/v1/people/match?linkedin_url={}&reveal_personal_emails=false&reveal_phone_number=false",
        encoded_url
    );

    dotenv().ok(); // load up .env file, same as "load_dotenv()" in python
    
    // Get API key from environment variable
    let api_key = std::env::var("APPOLO_1")
        .unwrap_or_else(|_| "ZfqWbaK2o1_vADiiy3R6IQ".to_string());
    
    // Create HTTP client
    let client = reqwest::Client::new();
    
    // Make the API call
    let response = client
        .post(&url)
        .header("accept", "application/json")
        .header("Cache-Control", "no-cache")
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .send()
        .await?;
    
    // Check if request was successful
    if !response.status().is_success() {
        println!("❌ Apollo API failed with status: {}", response.status());
        return Ok(String::new()); // Return empty string
    }
    
    // Parse JSON response
    let json: Value = response.json().await?;
    
    // Extract email directly
    let email = json["person"]["email"]
        .as_str()
        .unwrap_or("");
    
    if email.is_empty() {
        println!("❌ Apollo: No email found in response");
        Ok(String::new())
    } else {
        println!("✅ Apollo: Email found -> {}", email);
        Ok(email.to_string())
    }
}