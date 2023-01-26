use chrono::Utc;
use uuid::Uuid;

pub mod provider;
pub use provider::*;

use provider::provider_client::ProviderClient as Client;

pub use tonic::transport::Channel;
pub type ProviderClient = Client<Channel>;

impl List {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            icon: Some("✍️".to_string()),
        }
    }
}

impl Task {
    pub fn new(title: String, parent: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            parent,
            title,
            favorite: false,
            today: false,
            is_reminder_on: false,
            status: Status::NotStarted as i32,
            priority: Priority::Low as i32,
            sub_tasks: vec![],
            tags: vec![],
            notes: None,
            completion_date: None,
            deletion_date: None,
            due_date: None,
            reminder_date: None,
            reminder_time: None,
            recurrence: None,
            created_date_time: Utc::now().timestamp(),
            last_modified_date_time: Utc::now().timestamp(),
        }
    }
}
