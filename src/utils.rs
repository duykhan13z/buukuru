use ureq::{http::Response, Body};
use ureq::Agent;
use std::time::Duration;

pub fn fetch_from_internet(url: &str) -> Result<Response<Body>, ureq::Error> {
    println!("Downloading {}", url);

    let config = Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(3)))
        .timeout_global(Some(Duration::from_secs(3)))
        .https_only(true)
        .build();
    Agent::new_with_config(config).get(url)
    .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
    .call()
}