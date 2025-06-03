use reqwest;
use serde_json::Value;
use std::error::Error;
use std::collections::HashMap;
use dotenv::dotenv;

async fn get_access_token() -> Result<String, Box<dyn Error>> {
    dotenv().ok();
    
    // Get client credentials from environment
    let client_id = std::env::var("SNOV_CLIENT_ID").expect("Missing Snov.io client ID");
    let client_secret = std::env::var("SNOV_CLIENT_SECRET").expect("Missing Snov.io client secret");
    
    let client = reqwest::Client::new();
    
    // OAuth parameters
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id.as_str());
    params.insert("client_secret", client_secret.as_str());
    
    let response = client
        .post("https://api.snov.io/v1/oauth/access_token")
        .form(&params)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_body = response.text().await?;
        return Err(format!("Failed to get access token: {}", error_body).into());
    }
    
    let json: Value = response.json().await?;
    let access_token = json["access_token"]
        .as_str()
        .ok_or("No access token in response")?;
    
    Ok(access_token.to_string())
}

pub async fn get_email_from_linkedin(linkedin_url: &str) -> Result<String, Box<dyn Error>> {
    // STEP 0: Get fresh access token
    let access_token = get_access_token().await?;
    println!("✅ Snov.io: Got fresh access token");
    
    let client = reqwest::Client::new();
    
    // STEP 1: Add URL for search first
    let mut add_params = HashMap::new();
    add_params.insert("access_token", access_token.as_str());
    add_params.insert("url", linkedin_url);
    
    let add_response = client
        .post("https://api.snov.io/v1/add-url-for-search")
        .form(&add_params)
        .send()
        .await?;
    
    if !add_response.status().is_success() {
        let status_code = add_response.status();
        let error_body = add_response.text().await?;
        println!("❌ Snov.io add-url-for-search failed with status: {}", status_code);
        println!("❌ Full error response: {}", error_body);
        return Ok(String::new());
    }
    
    let _add_json: Value = add_response.json().await?;
    println!("✅ Snov.io: URL added for search");
    
    // STEP 2: Now get emails from URL
    let mut get_params = HashMap::new();
    get_params.insert("access_token", access_token.as_str());
    get_params.insert("url", linkedin_url);
    
    let response = client
        .post("https://api.snov.io/v1/get-emails-from-url")
        .form(&get_params)
        .send()
        .await?;
    
    // Check if request was successful
    if !response.status().is_success() {
        let status_code = response.status();
        let error_body = response.text().await?;
        println!("❌ Snov.io get-emails-from-url failed with status: {}", status_code);
        println!("❌ Full error response: {}", error_body);
        return Ok(String::new());
    }
    
    // Replace the email extraction section (lines 89-101) with:

    // Parse JSON response
    let json: Value = response.json().await?;

    // Print the full JSON response for debugging
    println!("🔍 Full Snov.io response: {}", serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string()));


    // Extract email from response - Snov.io has nested structure
    if let Some(data) = json["data"].as_array() {
        if let Some(person) = data.get(0) {
            if let Some(emails) = person["emails"].as_array() {
                if let Some(first_email) = emails.get(0) {
                    if let Some(email) = first_email["email"].as_str() {
                        let status = first_email["status"].as_str().unwrap_or("unknown");
                        println!("✅ Snov.io: Email found -> {} (status: {})", email, status);
                        return Ok(email.to_string());
                    }
                }
            }
        }
    }

    println!("❌ Snov.io: No email found in response");
    Ok(String::new())
}