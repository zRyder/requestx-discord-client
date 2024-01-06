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
