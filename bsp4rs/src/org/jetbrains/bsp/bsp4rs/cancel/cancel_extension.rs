use crate::*;

#[derive(Debug)]
pub enum CancelRequest {}

/// Like the language server protocol, a notification to ask the server to cancel a request.
impl Notification for CancelRequest {
    type Params = CancelRequestParams;
    const METHOD: &'static str = "$/cancelRequest";
}
