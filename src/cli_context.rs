use crate::request_method::RequestMethod;

pub struct CLIContext {
    pub request_method: RequestMethod,
    pub target_url: String,
    pub request_headers: Vec<RequestHeader>
}