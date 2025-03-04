use crate::infrastructure::network::plugin::Plugin;
use crate::infrastructure::logger::logger::Logger;
use reqwest::{Request, Response, Error};

pub struct CurlLoggerPlugin;

impl CurlLoggerPlugin {

    fn on_request_impl(&self, request: &Request) {
        let curl_command = CurlLoggerPlugin::request_to_curl(request);
        Logger::info(
            "[NETWORK]".to_string(), 
            &format!("Sending request: {}", curl_command)
        );
    }

    fn on_response_impl(&self, response: &Response) {
        Logger::info(
            "[NETWORK]".to_string(), 
            &format!("Received response with status: {}", response.status())
        );
    }

    fn on_error_impl(&self, error: &Error) {
        Logger::error(
            "[NETWORK]".to_string(), 
            &format!("Request occurred Error: {}", error.to_string())
        );
    }

    fn request_to_curl(request: &Request) -> String {
        let mut curl_command = String::new();
        curl_command.push_str("curl -X ");
        curl_command.push_str(request.method().as_str());
        curl_command.push_str(&format!(" '{}'", request.url()));

        for (name, value) in request.headers() {
            curl_command.push_str(&format!(" -H \"{}: {}\"", name, value.to_str().unwrap()));
        }

        if let Some(body) = request.body() {
            let body_str = if let Some(text) = body.as_bytes().and_then(|bytes| {
                std::str::from_utf8(bytes).ok()
            }) {
                text.replace('\'', "\\'")
            } else if let Some(chunk) = body.as_bytes() {
                format!("Binary Data ({:?})", chunk.iter().take(50).map(|&b|
                    format!("{:02X}", b)).collect::<Vec<_>>().join(" "))
            } else {
                String::from("Unknown Content")
            };

            if !body_str.is_empty() {
                curl_command.push_str(
                    &format!(" -d '{}'", body_str.replace('\'', "\\'")
                        .replace('\"', "\\\""))
                );
            }
        }

        curl_command
    }
}

impl Plugin for CurlLoggerPlugin {

    fn on_request(&self, request: &Request) {
        self.on_request_impl(request);
    }
    fn on_response(&self, response: &Response) {
        self.on_response_impl(response);
    }
    fn on_error(&self, error: &Error) {
        self.on_error_impl(error);
    }
}