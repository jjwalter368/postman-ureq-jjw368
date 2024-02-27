use ureq;
use serde_json::Value;


pub fn postman_ureq_json(url: &str) -> Value {
    let body: Value = serde_json::from_str(ureq::get(url).call().expect(".call failed").into_string().expect(".into_string failed").as_str()).expect("serde_json::from_str() failed");
    body
}
pub fn postman_ureq_str(url: &str) -> String {
    let body: String = ureq::get(url).call().expect(".call failed").into_string().expect(".into_string failed");
    body
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apple_captive() {
        let result = postman_ureq_str("https://captive.apple.com/");
        assert_eq!(result, "<HTML><HEAD><TITLE>Success</TITLE></HEAD><BODY>Success</BODY></HTML>\n");
    }
    #[test]
    fn server_test() {
        let result = postman_ureq_str("http://127.0.0.1:8000");
        assert_eq!(result, "Hello, world!");

    }
}
