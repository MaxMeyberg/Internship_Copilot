use reqwest;
use serde_json::Value;
use anyhow::{Result, Context, bail}; // Use anyhow instead
use dotenv::dotenv;

pub async fn get_email_from_linkedin(linkedin_url: &str) -> Result<String> {
    dotenv().ok(); // Load .env file
    
    // Get API key from environment variable
    let api_key = std::env::var("ZELIQ_API_KEY")
        .expect("Missing Zeliq API key");
    
    // Create HTTP client
    let client = reqwest::Client::new();
    
    // Create JSON body with LinkedIn URL
    let body = serde_json::json!({
        "linkedin_url": linkedin_url
    });
    
    // Make the API call (POST, not GET)
    let response = client
        .post("https://api.zeliq.com/api/contact/enrich/email")
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .header("x-api-key", api_key)
        .json(&body)
        .send()
        .await?;
    
    // Check if request was successful
    if !response.status().is_success() {
        let status_code = response.status();
        let error_body = response.text().await?;
        println!("❌ Full error response: {}", error_body);
        bail!("Zeliq API request failed with status: {}", status_code);
    }
    
    // Parse JSON response
    let json: Value = response.json().await.context("Failed to parse Zeliq response as JSON")?;
    
    // Print the full JSON response for debugging
    println!("🔍 Full Zeliq response: {}", serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string()));


    // Extract the most probable email from the contact object
    if let Some(contact) = json.get("contact") {
        if let Some(most_probable_email) = contact["most_probable_email"].as_str() {
            if !most_probable_email.is_empty() {
                let status = contact["most_probable_email_status"].as_str().unwrap_or("unknown");
                println!("✅ Zeliq: Most probable email found -> {} (status: {})", most_probable_email, status);
                return Ok(most_probable_email.to_string());
            }
        }
    }
    
    // ✅ Return proper error instead of empty string
    bail!("❌ Zeliq: No email found in response, ERROR OUT, issue on zeliq.rs");
}