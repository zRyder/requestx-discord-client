use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UpdateRequestManager {
	#[serde(rename = "duration")]
	pub duration_in_minutes: Option<u64>,
	pub enable_requests: Option<bool>,
	pub enable_gd_requests: Option<bool>
}
