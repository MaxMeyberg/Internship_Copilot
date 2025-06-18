use reqwest;
use serde_json::Value;
use anyhow::{Result, Context, bail};
use dotenv::dotenv;

//TODO: Cycle through the API keys automatically
pub async fn find_email(linkedin_url: &str) -> Result<String> {
    // URL encode the LinkedIn URL
    let encoded_url = urlencoding::encode(linkedin_url);
    
    // Build the API URL
    let url = format!(
        "https://api.apollo.io/api/v1/people/match?linkedin_url={}&reveal_personal_emails=false&reveal_phone_number=false",
        encoded_url
    );

    dotenv().ok(); // load up .env file
    
    // ✅ Get API key from environment variable (fixed name)
    let api_key = std::env::var("APPOLLO_API_KEY")
        .context("Missing APPOLLO_API_KEY environment variable")?;
    
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
        .await
        .context("Failed to send request to Apollo API")?;
    
    // ✅ Return proper error instead of empty string
    if !response.status().is_success() {
        let status_code = response.status();
        let error_body = response.text().await?;
        println!("❌ Apollo API failed with status: {}", status_code);
        println!("❌ Full error response: {}", error_body);
        bail!("Apollo API request failed with status: {}", status_code);
    }
    
    // Parse JSON response
    let json: Value = response.json().await
        .context("Failed to parse Apollo response as JSON")?;
    
    // Print debug info (matching Zeliq pattern)
    println!("🔍 Full Apollo response: {}", serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string()));
    
    // Extract email directly
    let email = json["person"]["email"]
        .as_str()
        .unwrap_or("");
    
    if email.is_empty() {
        // ✅ Return proper error instead of empty string
        println!("❌ Apollo: No email found in response");
        bail!("❌ Apollo: No email found in response, ERROR OUT, issue on appollo.rs");
    } else {
        println!("✅ Apollo: Email found -> {}", email);
        Ok(email.to_string())
    }
}