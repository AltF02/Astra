use crate::models::launch::LaunchID;
use crate::services::DB;
use sqlx::FromRow;
use std::default::Default;

#[derive(Default)]
pub struct Reminder {
    pub remaining: i64,
    pub launch: LaunchID,
}

#[derive(FromRow)]
pub struct User(pub i64);

impl From<&LaunchID> for Reminder {
    fn from(l: &LaunchID) -> Self {
        Reminder {
            launch: l.clone(),
            ..Default::default()
        }
    }
}

impl DB {
    pub async fn fetch_reminder_users(&self, reminder: Reminder) -> Vec<User> {
        sqlx::query_as("SELECT user_id FROM astra.reminders WHERE launch_id = $1")
            .bind(reminder.launch)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }
}
