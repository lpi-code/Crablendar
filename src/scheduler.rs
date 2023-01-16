// scheduler is the main loop of the program. It is responsible for
// scheduling the execution of tasks 

use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use std::{sync::Arc, env};
use crate::calendar::get_calendar;

pub async fn scheduler(cust_str: Arc<RwLock<String>>) {
    // calendar scheduler
    let calendar_future = calendar_scheduler(cust_str);
    // Join all task
    calendar_future.await

}

async fn calendar_scheduler(cust_str: Arc<RwLock<String>>) {
    loop {
        //
        // Change the string
        //env var
        let username = env::var("CRABLENDAR_USERNAME").unwrap_or_else(|_| { panic!("USERNAME env var not set") });
        let password = env::var("CRABLENDAR_PASSWORD").unwrap_or_else(|_| { panic!("PASSWORD env var not set") });
        let url = env::var("CRABLENDAR_URL").unwrap_or_else(|_| { panic!("URL env var not set") });
        let calendar = get_calendar(
            username.as_str(),
            password.as_str(),
            url.as_str()
        ).await.unwrap_or_else(|e| {
            println!("Error: {}", e);
            String::from("")
        });
        if calendar != "" {
            let mut data = cust_str.write().await;
            *data = calendar;
            println!("Calendar updated");
        }
        sleep(Duration::from_secs(600)).await;
    }
}