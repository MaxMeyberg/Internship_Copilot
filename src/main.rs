mod apify_call;
mod gpt;
mod appollo;
mod lead_iq;
mod zeliq;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let linkedin_url = "https://www.linkedin.com/in/satyanadella/".to_string();
    
    // Get data
    
    let apollo_email: String = zeliq::get_email_from_linkedin(&linkedin_url).await?;
    if apollo_email.is_empty() {
        println!("❌ No email found, generating LinkedIn message...");
        return Ok(());
    }

    let apify_json: Value = apify_call::run_actor(&linkedin_url).await?;


    
    // LLM Pipeline
    println!("🔄 Step 1: Parsing JSON data...");
    let parsed_data = gpt::generate_from_gpt("llm1_parse_json.txt", &apify_json.to_string()).await?;
    
    println!("🔄 Step 2: Creating strategy...");
    let strategy = gpt::generate_from_gpt("llm2_summarize_info.txt", &parsed_data).await?;
    
    println!("🔄 Step 3: Composing letter...");
    let letter_input = format!("{}\n\nVERIFIED EMAIL: {}", strategy, apollo_email);
    let letter = gpt::generate_from_gpt("llm3_compose_letter.txt", &letter_input).await?;
    
    println!("🔄 Step 4: Adding personality and formatting mailto...");
    let mailto_input = format!("{}\n\nVERIFIED EMAIL: {}", letter, apollo_email);
    let final_mailto = gpt::generate_from_gpt("llm4_add_personality_mailto.txt", &mailto_input).await?;
    
    println!("\n🎉 FINAL RESULT:\n{}", final_mailto);
    
    Ok(())
}