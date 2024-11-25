use chrono::{DateTime, Duration, Utc};

pub mod auth_error;
pub mod discord_user_error;
pub mod level_request_error;
pub mod level_review_error;

pub fn format_cooldown(cooldown_timestamp: DateTime<Utc>, request_cooldown: i64) -> Option<String> {
	let duration = (cooldown_timestamp + Duration::minutes(request_cooldown)) - Utc::now();

	let days = duration.num_days();
	let hours = duration.num_hours() - (days * 24);
	let minutes = duration.num_minutes() - (days * 1440 + hours * 60);
	let seconds = duration.num_seconds() - (days * 86400 + hours * 3600 + minutes * 60);

	// Display the duration
	let mut units = Vec::new();
	if days > 0 {
		units.push(format!("{} days", days));
	}
	if hours > 0 {
		units.push(format!("{} hours", hours));
	}
	if minutes > 0 {
		units.push(format!("{} minutes", minutes));
	}
	if seconds > 0 {
		units.push(format!("{} seconds", seconds));
	}

	let duration_str: Option<String>;

	if units.is_empty() {
		duration_str = None;
	} else {
		duration_str = if units.len() > 1 {
			let last_unit = units.pop().unwrap();
			let rest = units.join(", ");
			Some(format!("{} and {}", rest, last_unit))
		} else {
			Some(units.join(", "))
		};
	}
	duration_str
}
