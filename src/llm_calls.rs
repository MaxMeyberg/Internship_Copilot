use anyhow::{Result, Context}; // help with error handling
use serde_json::Value;
use std::{sync::Arc};
use crate::apis::{apify_call, apollo, zeliq, gpt}; // allows us to access gpt from our apis folder


#[derive(Debug, Clone)]
pub enum EmailAPI{
    Appollo, 
    Zeliq,
}

impl EmailAPI{
    // Finds the email
    pub async fn find_email(&self, linkedin_url : &str) -> Result<String> {
        match self {
            EmailAPI::Appollo => appollo::find_email(&linkedin_url).await?,
            EmailAPI::Zeliq => zeliq::find_email(&linkedin_url).await?,
            _ => anyhow::bail!("boi, you need an Appollo"),
             
        }
    }
}

pub struct InternCopilot{
    user_context: String,
    email_service: Arc<EmailAPI>,
}

impl InternCopilot {
    pub fn new(email_service: EmailAPI) -> Result<Self> {
        let about_me = fs::read_to_string("personal_context.txt").context("cant find file id called personal_content.txt")?; // THIS later on needs to be modified when project get bigger
        Ok(Self { // Does this return a tuple?
            about_me, 
            email_service: Arc::new(email_service), // This is different than my "new" in this function
        })
    }

    pub async fn generate_email(&self, linkedin_url: &str) -> Result<String> {
        let linkedin_url: Arc<String> = Arc::new(linkedin_url.to_string()); // we need to_strig for tokio to run

        let url1 = Arc::clone(&linkedin_url); // Clone the Arc pointer for child process 1
        let url2 = Arc::clone(&linkedin_url); // Clone the Arc pointer for child process 2

        let email_provider = Arc::clone(&self.email_service);

        // Concurrency TIME!!!!
        let email_handle = tokio::spawn( async move { // We need the "move" keyword since email_provider ends up owning the arc
            email_provider.find_email(&url1).await
        });

        let apify_handle = tokio::spawn( async move { // We need the "move" keyword since email_provider ends up owning the arc
            apfy_call::run_actor(&url2).await
        });

        //Concurrency END!!!
        let (email_res, json_res) = tokio::try_join!(email_handle, apify_handle).context("we fucked up the concurrency")?;

        let email = email_res.context("I dont think we found an email, from llm_calls.rs")?;
        let json: serde_json::Value = apify_result.context("RIP, Apify had an error getting Json datat, from llm_calls.rs")?;

        // LLM PIPELINE TIME
       

        /* TODO: Read me, written on (6/16/2025)

        Ask LLM on how to use this code snipept: IDK if its even correct:




        struct LLMPipeline<'a> {
        copilot: &'a InternCopilot,
        email: String,
        linkedin_data: serde_json::Value,
        }

        impl<'a> LLMPipeline<'a> {
            fn new(copilot: &'a InternCopilot, email: String, linkedin_data: serde_json::Value) -> Self {
                Self { copilot, email, linkedin_data }
            }

            async fn run(self) -> Result<String> {
                let parsed = self.parse_step().await?;
                let strategy = self.strategy_step(&parsed).await?;
                let letter = self.compose_step(&strategy, &parsed).await?;
                let final_email = self.personality_step(&letter).await?;
                Ok(final_email)
            }
        }
            
         */

        

        

    }
}
