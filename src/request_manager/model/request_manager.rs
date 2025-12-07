use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestConfig {
	#[serde(rename = "duration")]
	pub duration_in_minutes: Option<u64>,
	pub enable_requests: Option<bool>,
	pub enable_gd_requests: Option<bool>,
	pub allow_non_user_created_levels: Option<bool>,
}

impl RequestConfig {
	pub fn new(
		duration_in_minutes: Option<u64>,
		enable_requests: Option<bool>,
		enable_gd_requests: Option<bool>,
		allow_non_user_created_levels: Option<bool>,
	) -> Self {
		Self {
			duration_in_minutes,
			enable_requests,
			enable_gd_requests,
			allow_non_user_created_levels,
		}
	}
}
