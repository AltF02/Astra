pub mod common;
pub mod crew;
pub mod event;
pub mod launch;
pub mod launcher;
pub mod manufacturer;
pub mod mission;
pub mod rocket;
pub mod spacecraft;
pub mod traits;
pub mod url;

#[cfg(debug_assertions)]
pub const BASE_URL: &str = "https://ll.thespacedevs.com/2.0.0";

#[cfg(not(debug_assertions))]
pub const BASE_URL: &str = "https://ll.thespacedevs.com/2.0.0";
