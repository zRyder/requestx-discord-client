use chrono::{DateTime, Utc};
use crate::level_request::model::level_request::LevelRequest;

#[derive(Debug, Clone)]
pub struct LevelRequestModalRequest {
    pub level_request: LevelRequest,
    pub expiry: DateTime<Utc>
}

impl LevelRequestModalRequest {
    pub fn new(level_request: LevelRequest, expiry: DateTime<Utc>) -> Self {
        Self {
            level_request,
            expiry
        }
    }
    
    pub fn is_valid_request(&self, now: DateTime<Utc>) -> bool {
        self.expiry > now
    }
}