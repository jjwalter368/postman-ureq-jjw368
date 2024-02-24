use ureq;

pub fn postman_ureq(url: &str) -> String {
    let body: String = ureq::get(url).call().expect(".call failed").into_string().expect(".into_string failed");
    body
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apple_captive() {
        let result = postman_ureq("https://captive.apple.com/");
        assert_eq!(result, "<HTML><HEAD><TITLE>Success</TITLE></HEAD><BODY>Success</BODY></HTML>\n");
    }
    //fn server_test() {
        //when api works, use the url from that
        //let result = postman_ureq("http://127.0.0.1/8800");
        //assert_eq!(result, "<HTML><HEAD><TITLE>Success</TITLE></HEAD><BODY>Success</BODY></HTML>\n");

    //}
}
