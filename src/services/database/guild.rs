#[derive(Debug)]
pub struct DBGuild {
    pub guild_id: i64,
    pub channel_id: i64,
    pub active: bool,
    pub launches: bool,
    pub apod: bool,
    pub events: bool,
}
