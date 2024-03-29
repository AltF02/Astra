// The spacedevs api URL
#[cfg(debug_assertions)]
pub const BASE_URL: &str = "https://lldev.thespacedevs.com/2.2.0";

#[cfg(not(debug_assertions))]
pub const BASE_URL: &str = "https://ll.thespacedevs.com/2.2.0";

pub const APOD_URL: &str = "https://api.nasa.gov/planetary/apod?api_key=";

pub const PLACEHOLDER: &str =
    "https://launchlibrary1.nyc3.digitaloceanspaces.com/RocketImages/placeholder_1920.png";
