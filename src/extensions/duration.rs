use chrono::Duration;

pub trait DurationExt {
    fn create_24h(&self) -> String;
}

impl DurationExt for Duration {
    fn create_24h(&self) -> String {
        let mins = (self.num_minutes() - 60 * self.num_hours()).to_string();
        let min = if mins.len() == 1 {
            format!("0{}", mins)
        } else {
            mins
        };
        let hour = if self.num_hours().to_string().len() == 1 {
            format!("0{}", self.num_hours())
        } else {
            self.num_hours().to_string()
        };
        format!("{}:{}", hour, min)
    }
}
