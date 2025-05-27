mod apify_call;
mod gpt;
use anyhow::{Result, Context};
use std::path::Path;
use tokio::fs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

   
    // TODO: Change this url to a user input
    let linkedin_url = "https://www.linkedin.com/in/jmartling/".to_string();
    // TODO: Check to see if url is valid


    /*❓ Need help understanding API call? 👉 Click me! 🖱️ 

    The "?" are simple shorthand to be:

    match apify_call::run_actor(&linkedin_url).await {
        Ok(data) => {
            println!("JSON: {}", serde_json::to_string_pretty(&data)?);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
    */
    let apify_data = apify_call::run_actor(&linkedin_url).await?;
    
    println!("JSON: {}", serde_json::to_string_pretty(&apify_data)?);
    let json_str = apify_data.to_string();
    println!("\n→ ProfileInfo: {:#?}", json_str);
    // get the system prompt from .txt file
    let system_prompt = fs::read_to_string(Path::new("system_prompt.txt"))
        .await
        .context("Failed to read system prompt file")?;

    // FINI

    let gpt_response = gpt::generate_from_gpt(&system_prompt, &json_str).await?;

    
    println!("\n=== GENERATED EMAIL ===\n{}", gpt_response.to_string());

    //let test = "mailto:contact@company.com?subject=Job%20Application&body=Hello%2C%0A%0AI%20saw%20your%20job%20posting%20and%20would%20like%20to%20apply.".to_string();
    Ok(())


    //TODO: Check emails if none, then show it cant find an email and then tailor a linkedin message



}