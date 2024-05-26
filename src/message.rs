use std::error::Error;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    date: Option<String>,
    time: Option<String>,
    user_name: String,
    message: String
}

impl Message {
    pub fn new(user_name: String, message: String) -> Result<Self, Box<dyn Error>> {
        let local_time = Local::now();
        let date = Some(local_time.to_utc().format("%d/%m/%Y").to_string()); 
        let time = Some(local_time.time().format("%H:%M:%S").to_string());

        Ok(Self { date, time, user_name, message })
    }
    
    pub fn date(&self) -> &Option<String> {
        &self.date
    }
    pub fn time(&self) -> &Option<String> {
        &self.time
    }
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}