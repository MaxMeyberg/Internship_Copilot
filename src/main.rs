pub mod apis;
use apis::{gpt, apify_call, zeliq, appollo};
use serde_json::{Value};
use serde_json::to_string_pretty;
use std::env;
use std::fs;
use std::io::{self, Write};
use anyhow::{Context, Result};
use colored::*;
use std::collections::HashMap; // for parsing


#[tokio::main]



async fn main() -> Result<(), Box<dyn std::error::Error>>{
    //----------
    let args: Vec<String> = env::args().collect();
    
    // Check if LinkedIn URL was provided
    // TODO: This feels like bloat code
    let linkedin_url = match args.get(1) {
        Some(url) => url.clone(),
        None => {
            println!("❌ Please provide a LinkedIn URL as an argument");
            println!("Usage: cargo run -- \"https://www.linkedin.com/in/username/\"");
            return Ok(());
        }
    };
    //----------
    // Get the email to send to
    let email_adr: String = get_email(&linkedin_url).await?;

    let apify_json: Value = apify_call::run_actor(&linkedin_url).await?;
    
    // let experiences = &apify_json.get("experiences");
    // println!("{:?}", experiences);

    let parsed = parse_json(&apify_json);
    
    // LLM Pipeline
    println!("🔄 Step 1: Parsing JSON data...");
    let parsed_data = gpt::generate_from_gpt("llm1_parse_json.txt", &parsed).await?;
    
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
// TODO: Create better emails, the first part sounds weird, too much sycophancy!!!
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
    let found_email = appollo::find_email(&linkedin_url).await?;


    if found_email.is_empty(){
        anyhow::bail!("No email we could find, RIP"); // Auto panic if no email
    }
    Ok(found_email)

}


//TODO: add this in a seperate fle and folder, call the folder "tools"
fn parse_json(apify_json: &Value) -> String {
    let mut dic = HashMap::new();
    let first_name =  &apify_json.get("firstName").and_then(|x| x.as_str()).unwrap_or("");
    let last_name =  &apify_json.get("lastName").and_then(|x| x.as_str()).unwrap_or("");
    dic.insert("firstName".to_string(), first_name.to_string());
    dic.insert("lastName".to_string(), last_name.to_string());
    
    let email = &apify_json.get("about").and_then(|x| x.as_str()).unwrap_or("");
    let company = &apify_json.get("companyName").and_then(|x| x.as_str()).unwrap_or("");
    dic.insert("email".to_string(), email.to_string());
    dic.insert("company".to_string(), company.to_string());

    // Experiences is pain to parse, id compress this for your sanity
    if let Some(experiences) = apify_json.get("experiences").and_then(|x| x.as_array()) { // get list of expeirnces, JSON is messy atm
        let mut count = 1; // used for new keys on each experience
        for exp in experiences {
            if let Some(subs) = exp.get("subComponents").and_then(|x| x.as_array()) { // look at subcomponents
                for sub in subs { // loop each subcomponent
                    let title = sub.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    // Join all "text" fields in the "description" array
                    let description = sub.get("description")
                        .and_then(|desc_arr| desc_arr.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|d| d.get("text").and_then(|t| t.as_str())) // Tries to get each "text" in the json to convert to a &str so it can be copied down
                                .collect::<Vec<_>>() // turn all strings into a vector
                                .join(" ")
                        }) // combines the JSON Mess into a string thats readable
                        .unwrap_or_default(); // confirms if value exists or not for error handling
                    let combined = format!("{}: {}", title, description);
                    let key = format!("experience{}", count);
                    dic.insert(key, combined);
                    count += 1;
                }
            }
        }
    }
    let res = serde_json::to_string_pretty(&dic).unwrap_or_default();

    //TODO: Add in 
    res
}