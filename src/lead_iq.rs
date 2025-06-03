use reqwest;
use serde_json::Value;
use std::error::Error;
use::dotenv::dotenv;

//TODO: Cycle through the API keys automatically
pub async fn get_email_from_linkedin(linkedin_url: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok(); // load up .env file, same as "load_dotenv()" in python
    
    // Use LeadIQ GraphQL endpoint
    let url = "https://api.leadiq.com/graphql";
    
    // Get LeadIQ API key from environment variable
    let api_key = std::env::var("LEADIQ_API_KEY1")
        .unwrap_or_else(|_| "".to_string());
    
    // Use the correct SearchPeople query format
    let query = r#"
        query SearchPeople($input: SearchPeopleInput!) {
          searchPeople(input: $input) {
            totalResults
            hasMore
            results {
              _id
              name {
                first
                fullName
                last
              }
              currentPositions {
                title
                emails {
                  type
                  status
                  value
                }
                companyInfo {
                  name
                }
              }
              linkedin {
                linkedinUrl
              }
              personalEmails {
                type
                status
                value
              }
            }
          }
        }
    "#;
    
    // Create GraphQL request body - search by LinkedIn URL
    let body = serde_json::json!({
        "query": query,
        "variables": {
            "input": {
                "linkedinUrl": linkedin_url
            }
        }
    });
    
    // Create HTTP client
    let client = reqwest::Client::new();
    
    // Make the API call (remove Bearer prefix based on your curl test)
    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Basic {}", api_key))  // ✅ Correct auth method
        .json(&body)
        .send()
        .await?;
    
    // Check if request was successful
    if !response.status().is_success() {
        let status_code = response.status();
        let error_text = response.text().await?;
        println!("❌ LeadIQ API failed with status: {}", status_code);
        println!("❌ Error details: {}", error_text);
        return Ok(String::new());
    }
    
    // Parse JSON response
    let json: Value = response.json().await?;
    
    // Check for GraphQL errors
    if let Some(errors) = json.get("errors") {
        println!("❌ LeadIQ GraphQL errors: {}", errors);
        return Ok(String::new());
    }
    
    // Extract email from searchPeople response (Fixed path)
    let total_results = json["data"]["searchPeople"]["totalResults"].as_u64().unwrap_or(0);
    
    if total_results == 0 {
        println!("❌ LeadIQ: No person found");
        return Ok(String::new());
    }
    
    // Get first result (Fixed path)
    let results = &json["data"]["searchPeople"]["results"];
    if let Some(person) = results.get(0) {
        // Try work emails from current positions first
        if let Some(positions) = person["currentPositions"].as_array() {
            for position in positions {
                if let Some(emails) = position["emails"].as_array() {
                    for email_obj in emails {
                        if let Some(email) = email_obj["value"].as_str() {
                            if !email.is_empty() {
                                println!("✅ LeadIQ: Work email found -> {}", email);
                                return Ok(email.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        // Try personal emails as fallback
        if let Some(emails) = person["personalEmails"].as_array() {
            for email_obj in emails {
                if let Some(email) = email_obj["value"].as_str() {
                    if !email.is_empty() {
                        println!("✅ LeadIQ: Personal email found -> {}", email);
                        return Ok(email.to_string());
                    }
                }
            }
        }
    }
    
    println!("❌ LeadIQ: No email found in response");
    Ok(String::new())
}