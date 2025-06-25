/// This file is meant to parse the fat json from Apify and make it focus on the main points we want
/// The Apify Json is a Titanic Mess, so the code needs to dig into these 
/// Recurssive nested jsons, thus why the code is so messy
/// 

use serde_json::{Value};
use std::collections::HashMap; // for parsing



//TODO: add this in a seperate fle and folder, call the folder "tools"
pub fn apify_json(apify_json: &Value) -> String {
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