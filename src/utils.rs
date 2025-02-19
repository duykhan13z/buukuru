use ureq::{http::Response, Body};
use ureq::Agent;
use std::thread;
use std::time::Duration;

pub fn fetch_from_internet(url: &str) -> Result<Response<Body>, ureq::Error> {
    let max_retries: u32 = 3;
    let retry_delay: Duration = Duration::from_secs(5);

    println!("Downloading {}", url);

    let config = Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(3)))
        .timeout_global(Some(Duration::from_secs(3)))
        .https_only(true)
        .build();
    let agent = Agent::new_with_config(config);

    let mut retries = 0;

    while retries < max_retries {
        match agent.get(url)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
            .call() {
            Ok(response) => return Ok(response),
            Err(e) => match e {
                ureq::Error::Timeout(_) => {
                    retries += 1;
                    println!("Request timed out. Retrying ({}/{}).", retries, max_retries);
                    thread::sleep(retry_delay);
                }
                _ => return Err(ureq::Error::ConnectionFailed),
            }
        }
    }

    Err(ureq::Error::ConnectionFailed)
}

