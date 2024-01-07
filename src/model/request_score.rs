use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize, Serializer};

#[derive(PartialEq, Deserialize, Debug)]
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

impl Display for RequestRating {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			RequestRating::One => { write!(f, "Auto, One Star/Moon") }
			RequestRating::Two => {write!(f, "Easy, Two Stars/Moons")}
			RequestRating::Three => {write!(f, "Normal, Three Stars/Moons")}
			RequestRating::Four => {write!(f, "Hard, Four Stars/Moons")}
			RequestRating::Five => {write!(f, "Hard, Five Stars/Moons")}
			RequestRating::Six => {write!(f, "Harder, Six Stars/Moons")}
			RequestRating::Seven => {write!(f, "Harder, Seven Stars/Moons")}
			RequestRating::Eight => {write!(f, "Insane, Eight Stars/Moons")}
			RequestRating::Nine => {write!(f, "Insane, Nine Stars/Moons")}
			RequestRating::Ten => {write!(f, "Demon, Ten Stars/Moons")}
		}
	}
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
