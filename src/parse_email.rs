use serde_json::Value;
use colored::Colorize;


///This will parse the raw json to find if a email is ehrre
fn parse_email(json: &Value) -> String {
    let email = json[0]["email"].as_str();
    email
}