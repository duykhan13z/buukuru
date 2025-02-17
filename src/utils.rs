use ureq::{http::Response, Body};

pub fn fetch_from_internet(url: &str) -> Result<Response<Body>, ureq::Error> {
    ureq::get(url)
    .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
    .call()
}