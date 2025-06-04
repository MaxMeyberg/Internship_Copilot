/* ❓ Need help understanding API call? 👉 Click me! 🖱️ 
Before you suffer, This is the Python Apify API documentaion, we wanna mimick this in Rust:

from apify_client import ApifyClient

# Initialize the ApifyClient with your API token
client = ApifyClient("<YOUR_API_TOKEN>")

# Prepare the Actor input
run_input = { "profileUrls": [
        "https://www.linkedin.com/in/williamhgates",
        "http://www.linkedin.com/in/jeannie-wyrick-b4760710a",
    ] }

# Run the Actor and wait for it to finish
run = client.actor("2SyF0bVxmgGr8IVCZ").call(run_input=run_input)

# Fetch and print Actor results from the run's dataset (if there are any)
item = next(client.dataset(run["defaultDatasetId"]).iterate_items())


*/

use reqwest::Client; //Rust library needed to have rust send HTTP requests
use serde_json::{json, Value}; // Allows "let url = json!({"profileUrls": [linkedin_url]});" to work
use dotenv::dotenv; // we cn access .env file
use anyhow::{Result, Context}; // adds more detailed errors, we dont need std::error anymore
use colored::Colorize; // colors in print statements :)
use std::sync::Arc;


struct ApifyAPI {
    api_key: String, 
    actor_id: String,
    api_url: String, // URL for apify API
}

impl ApifyAPI {
    /// This is a contructor to neatly pack all the data from apify into an easier API call
    fn new() -> Self{
        
        dotenv().ok(); // load up .env file, same as "load_dotenv()" in python
        let api_key: String = std::env::var("APIFY_API_KEY").expect("Missing Apify API key, is it gone?"); // get api key from /.env, panic if no API key found
        let actor_id: String = "2SyF0bVxmgGr8IVCZ".to_string(); // change actor_id to change web scraper we use
        let api_url: String = format!("https://api.apify.com/v2/acts/{}/runs", actor_id);
        
        ApifyAPI { api_key, actor_id, api_url,} // return the default
    }

    /* ❓ Need help understanding? 👉 Click me! 🖱️
    Let's break down:
    Result<serde_json::Value, Box<dyn Error>>

    Result<> is an enum that is Value or Err

    Rust requires all types to have a known size at compile time, so for every error type, we need to say its dyn or the borrow checker is after our cheeks

    AKA: the "Err" is Box<dyn error::Error>

    the std::error is this super fat struct that hurts my head, but there is stuff like std::error::Report, std::error::Error and stuff like that

    Box<> just mean its heap data, ngl I am still having a hard time fullly understnding this concept I am explaining
     */
    
    async fn post_request(&self, linkedin_url: &str) -> Result<serde_json::Value> {
        // Initialize the HTTP client
        let client = Client::new();
        //DONT modify "profileUrls": or else the Apify API wont work

        let json_url: Value = json!({"profileUrls": [linkedin_url]});

        /* ❓ Need help understanding? 👉 Click me! 🖱️

            .post(&self.url) -> RequestBuilder 

                create the HTTP POST request and make it target the Apify URL we want

            .bearer_auth(&self.api_key) -> RequestBuilder 

                Adds a Bearer token (using the API key) to the Authorization header of HTTP request for authentication.
                (AKA: Ass in the holder of the API key)

            .json(&json_url) -> RequestBuilder 

                Add in the JSON payload (AKA: just the linkedin URL)

            .send().await

                Simply sends and waits for the request, 
                in the meantime, lets keep running on other things in rust while the http request is being sent

            .context("ERROR message I wanna say")?

                uses anyhow Crate to allow us to write clean code
                -----[BEFORE]--------
                .await {
                Ok(resp) => resp,
                Err(e) => return Err(Box::new(CustomError {
                    message: format!("Failed to run the Actor: {}", e),
                    source: Some(Box::new(e))
                }))
                };

                -----[AFTER]--------
                
                .await
                .context("Failed to run the Actor")?


            TLDR, shorthand for cleaner code :D
        */
        let response = client
            /*Http Post Request*/.post(&self.api_url) 
            /*Http Post Request*/.bearer_auth(&self.api_key)
            .json(&json_url)
            .send()
            .await
            .context("Failed to run the Actor")?; /* ❓ Need help understanding?
        
         */ 
        // Parse the JSON Response from API call, Apify should confirm that they got our stuff
        let json_receipt: serde_json::Value = response.json().await?;
        /* Need help understanding? 👉 Click me! 🖱️
        Since we return:

        enum Result<T, E> {
            Ok(T),  // Represents a successful result containing a value of type `T`
            Err(E), // Represents an error result containing a value of type `E`
        }

        We need the Ok() to be
        */
        println!("{}", "post_request sent over, waiting for webhook notification".yellow());
        Ok(json_receipt)

    }
    
}
///Testing here for docs
pub async fn run_actor(profile_url: &str) -> Result<serde_json::Value> {

    // Create a new Arc, which creates a new ApifyAPI struct
    let apify = Arc::new(ApifyAPI::new());

    // Make the API call to run the Actor (POST)
    let run = apify.post_request(profile_url).await?;
    //TODO Add in manual polling


    // Retry mechanism to wait for the actor to finish
    let client = Client::new();
    

    // Retry mechanism to wait for the actor to finish
    let mut retries = 10;
    let mut run_status = run.clone();

    while retries > 0 {
        if run_status["data"]["finishedAt"].as_str().is_some() {
            break;
        } else {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            //(GET)
            
            let run_status_url = format!("https://api.apify.com/v2/acts/{}/runs/{}", apify.actor_id, run["data"]["id"].as_str().unwrap());
            let run_status_response = client
                .get(&run_status_url)
                .bearer_auth(apify.api_key.clone())
                .send()
                .await?;

            if run_status_response.status().is_success() {
                run_status = run_status_response.json().await?;
            } else {
                return Err(anyhow::anyhow!("Failed to fetch run status"));
            }
        }

        retries -= 1;
    }

    if retries == 0 {
        return Err(anyhow::anyhow!("Actor did not finish in time"));
    }

    // Use match instead of if-else for handling dataset_id
    match run["data"]
    .get("defaultDatasetId")
    .and_then(|v| v.as_str())
    {
        Some(dataset_id) => {
            let dataset_url =
                format!("https://api.apify.com/v2/datasets/{}/items", dataset_id);
            let resp = client
                .get(&dataset_url)
                .bearer_auth(&apify.api_key)
                .send()
                .await?;
            resp.error_for_status_ref()
                .context("Failed to fetch dataset")?;
            
            // Parse into a Vec<Value>
            let items: Vec<serde_json::Value> = resp.json().await?;
            
            // Return the *raw* first item (no field extraction)
            if let Some(first) = items.into_iter().next() {
                return Ok(first);
            } else {
                return Err(anyhow::anyhow!("Dataset is empty"));
            }
        }
        None => return Err(anyhow::anyhow!("Dataset ID not found")),
    }
}
