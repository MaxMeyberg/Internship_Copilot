
// Allows you to get email, either manually or via Zeliq
use anyhow::{Context, Result}; // for erro rhandling
use crate::apis::appollo;
use std::io::{self, Write};

/// Asks the user for a manual email input, if not, it uses appollo to get he email for you
pub async fn get_email(linkedin_url: &str) -> Result<String> {

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