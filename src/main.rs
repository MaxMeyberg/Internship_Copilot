pub mod apis;
use apis::{gpt, apify_call, zeliq};
use serde_json::Value;
use std::env;
use std::fs;
use std::io::{self, Write};
use anyhow::{Context, Result};

#[tokio::main]


//TODO: Add comand line argument

async fn main() -> Result<(), Box<dyn std::error::Error>>{
    //----------
    let args: Vec<String> = env::args().collect();
    
    // Check if LinkedIn URL was provided
    // Check if LinkedIn URL was provided using match
    let linkedin_url = match args.get(1) {
        Some(url) => url.clone(),
        None => {
            println!("❌ Please provide a LinkedIn URL as an argument");
            println!("Usage: cargo run -- \"https://www.linkedin.com/in/username/\"");
            return Ok(());
        }
    };
    //----------

    // TODO: Add in personal linkedin to ad on with 
    
    // Get data
    let email_adr: String = get_email(&linkedin_url).await?;

    let apify_json: Value = apify_call::run_actor(&linkedin_url).await?;

    // LLM Pipeline
    println!("🔄 Step 1: Parsing JSON data...");
    let parsed_data = gpt::generate_from_gpt("llm1_parse_json.txt", &apify_json.to_string()).await?;
    
    println!("🔄 Step 2: Creating strategy... How can I relate to this person");
    let personal_context = fs::read_to_string("personal_context.txt").context("Cant read your personal info file")?;
    let strategy_input = format!(
        "TARGET PERSON'S LINKEDIN DATA:\n{}\n\n--- SEPARATOR ---\n\nMY PERSONAL CONTEXT:\n{}", 
        parsed_data, 
        personal_context
    );
    let strategy = gpt::generate_from_gpt("llm2_summarize_info.txt", &strategy_input).await?;


    
    println!("🔄 Step 3: Composing letter...");
    let letter_input = format!("{}\n\nVERIFIED EMAIL: {}", strategy, email_adr);
    let letter = gpt::generate_from_gpt("llm3_compose_letter.txt", &letter_input).await?;

    println!("🔄 Step 4: Adding personality and formatting mailto...");
    let mailto_input = format!("{}\n\nVERIFIED EMAIL: {}", letter, email_adr);
    let final_mailto = gpt::generate_from_gpt("llm4_add_personality_mailto.txt", &mailto_input).await?;
    
    println!("\n🎉 FINAL RESULT:\n{}", final_mailto);
    
    Ok(())
}


// Allows you to get email, either manually or via Zeliq
async fn get_email(linkedin_url: &str) -> Result<String> {

    print!("📧 Type in Email (Press Enter to auto-find): ");
    io::stdout().flush().context("Failed to flush stdout")?; // ✅ Handle errors properly

    fn read_user_input() -> Result<String> { 
        let mut input = String::new();
        io::stdin().read_line(&mut input).context("Failed to read user input")?;
        Ok(input.trim().to_string()) // makes the input remove the /n and also pass borrow checker w string type
    }

    let input = read_user_input()?;

    

    if !input.is_empty() {
        return Ok(input)
    } 

    // Auto-find email using Zeliq
    println!("🔄 Auto-finding email...");
    let found_email = zeliq::get_email_from_linkedin(&linkedin_url).await?;


    if found_email.is_empty(){
        anyhow::bail!("No email we could find, RIP"); // Auto panic if no email
    }
    Ok(found_email)

}