mod apify_call;
mod gpt;
use serde_json::Value;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

   
    // TODO: Change this url to a user input
    let linkedin_url = "https://www.linkedin.com/in/williamhgates/".to_string();
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
    let json: Value = apify_call::run_actor(&linkedin_url).await?; 

    
    

    // TODO: Parse email from JSON
    let email = parse_email(&json);

    // Check if email is there or not, that determines the type of prompt:
    let system_prompt = match email{
        "" => "linkedin_prompt.txt", 
        _ => "email_prompt.txt"
    };

    

    let gpt_response = gpt::generate_from_gpt(system_prompt, &json.to_string()).await?;

    
    println!("\n=== GENERATED EMAIL ===\n{}", gpt_response.to_string());

    //let test = "mailto:contact@company.com?subject=Job%20Application&body=Hello%2C%0A%0AI%20saw%20your%20job%20posting%20and%20would%20like%20to%20apply.".to_string();
    Ok(())


    //TODO: Check emails if none, then show it cant find an email and then tailor a linkedin message



}

///This will get the email from the Json, to see if its theres an email
fn parse_email(json: &Value) -> &str {
    
    let email: Option<&str> = json["email"].as_str(); // Convert Json to Option<&str>
    let email: &str = email.unwrap_or(""); // Unwrap the Option<&str> to become a &str
    
    match email {
        "" => {
            println!("❌ No email found - generating LinkedIn message");
            email
        },
        valid_email => {
            println!("✅ Email found: {}", valid_email);
            valid_email
        }

    }

}