pub mod discord;
pub mod model;
pub mod service;

pub fn format_cooldown_duration_string(cooldown_duration_in_minutes: u64) -> Option<String> {
	let mut content = String::new();
	if cooldown_duration_in_minutes == 0 {
		return None;
	} else {
		let days = cooldown_duration_in_minutes / 1440;
		let hours = (cooldown_duration_in_minutes % 1440) / 60;
		let minutes = cooldown_duration_in_minutes % 60;
		let mut time_parts = Vec::new();

		push_time_unit(&mut time_parts, days, "day", "days");
		push_time_unit(&mut time_parts, hours, "hour", "hours");
		push_time_unit(&mut time_parts, minutes, "minute", "minutes");

		let duration_str = if time_parts.len() > 1 {
			let last_unit = time_parts.pop().unwrap();
			let remaining_time_units = time_parts.join(", ");
			format!("{} and {}", remaining_time_units, last_unit)
		} else {
			time_parts.join(", ")
		};
		content.push_str(&format!("{}", duration_str));
	}

	Some(content)
}

fn push_time_unit(time_parts: &mut Vec<String>, value: u64, singular: &str, plural: &str) {
	if value == 1 {
		time_parts.push(format!("**1 {}**", singular));
	} else if value > 1 {
		time_parts.push(format!("**{} {}**", value, plural));
	}
}
