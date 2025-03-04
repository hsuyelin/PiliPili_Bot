use reqwest::{Request, Response, Error};

pub trait Plugin {
    fn on_request(&self, request: &Request);
    fn on_response(&self, response: &Response);
    fn on_error(&self, error: &Error);
}