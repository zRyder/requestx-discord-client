use std::str::FromStr;

use serde::{Serialize, Serializer};

#[derive(PartialEq)]
pub enum RequestRating {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten
}

impl FromStr for RequestRating {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"One" => Ok(RequestRating::One),
			"Two" => Ok(RequestRating::Two),
			"Three" => Ok(RequestRating::Three),
			"Four" => Ok(RequestRating::Four),
			"Five" => Ok(RequestRating::Five),
			"Six" => Ok(RequestRating::Six),
			"Seven" => Ok(RequestRating::Seven),
			"Eight" => Ok(RequestRating::Eight),
			"Nine" => Ok(RequestRating::Nine),
			"Ten" => Ok(RequestRating::Ten),
			_ => Err(())
		}
	}
}

impl Serialize for RequestRating {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		match self {
			RequestRating::One => serializer.serialize_str("One"),
			RequestRating::Two => serializer.serialize_str("Two"),
			RequestRating::Three => serializer.serialize_str("Three"),
			RequestRating::Four => serializer.serialize_str("Four"),
			RequestRating::Five => serializer.serialize_str("Five"),
			RequestRating::Six => serializer.serialize_str("Six"),
			RequestRating::Seven => serializer.serialize_str("Seven"),
			RequestRating::Eight => serializer.serialize_str("Eight"),
			RequestRating::Nine => serializer.serialize_str("Nine"),
			RequestRating::Ten => serializer.serialize_str("Ten")
		}
	}
}
