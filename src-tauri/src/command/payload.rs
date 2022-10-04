// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}

#[derive(Clone, serde::Serialize)]
pub struct PayloadDownload {
    pub download_type: String,
    pub message: String,
    pub total: usize,
    pub current: usize,
}
