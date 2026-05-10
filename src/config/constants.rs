use chrono::Duration;

pub static CONTENT_TYPE: &'static str = "Content-Type";
pub static CONTENT_LENGTH: &'static str = "Content-Length";
pub static APPLICATION_JSON: &'static str = "application/json";
pub static YOUTUBE_LINK_REGEX: &'static str = "^((?:https?:)?\\/\\/)?((?:www|m)\\.)?((?:youtube(-nocookie)?\\.com|youtu.be))(\\/(?:[\\w\\-]+\\?v=|embed\\/|v\\/)?)([\\w\\-]+)(\\S+)?$";
pub static EMPTY_STRING: String = String::new();
pub static LEVEL_REQUEST_MODAL_REQUEST_VALID_UNTIL: Duration = Duration::minutes(1);
