use std::str::FromStr;

use serde::{Deserialize, Serialize, Serializer};

use crate::level_request::model::level_request::LevelRequest;

#[derive(Deserialize, Debug, Clone)]
pub struct SentLevel {
	pub level_request: LevelRequest,
	pub moderator_data: ModeratorData
}

#[derive(Serialize, Debug, Copy, Clone)]
pub struct SendLevelRequest {
	pub level_id: u64,
	#[serde(flatten)]
	pub moderator_data: ModeratorData
}

impl SendLevelRequest {
	pub fn new(
		level_id: u64,
		suggested_score: SuggestedScore,
		suggested_rating: SuggestedRating
	) -> Self {
		Self {
			level_id,
			moderator_data: ModeratorData {
				suggested_score,
				suggested_rating
			}
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ModeratorData {
	pub suggested_score: SuggestedScore,
	pub suggested_rating: SuggestedRating
}

#[derive(PartialEq, Deserialize, Debug, Copy, Clone)]
pub enum SuggestedScore {
	NoRate,
	Rated,
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

#[derive(PartialEq, Deserialize, Debug, Copy, Clone)]
pub enum SuggestedRating {
	Rate,
	Feature,
	Epic,
	Legendary,
	Mythic
}

impl FromStr for SuggestedScore {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"NoRate" => Ok(Self::NoRate),
			"Rated" => Ok(Self::Rated),
			"One" => Ok(Self::One),
			"Two" => Ok(Self::Two),
			"Three" => Ok(Self::Three),
			"Four" => Ok(Self::Four),
			"Five" => Ok(Self::Five),
			"Six" => Ok(Self::Six),
			"Seven" => Ok(Self::Seven),
			"Eight" => Ok(Self::Eight),
			"Nine" => Ok(Self::Nine),
			"Ten" => Ok(Self::Ten),
			_ => Err(())
		}
	}
}

impl Serialize for SuggestedScore {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		match self {
			SuggestedScore::NoRate => serializer.serialize_str("NoRate"),
			SuggestedScore::Rated => serializer.serialize_str("Rated"),
			SuggestedScore::One => serializer.serialize_str("One"),
			SuggestedScore::Two => serializer.serialize_str("Two"),
			SuggestedScore::Three => serializer.serialize_str("Three"),
			SuggestedScore::Four => serializer.serialize_str("Four"),
			SuggestedScore::Five => serializer.serialize_str("Five"),
			SuggestedScore::Six => serializer.serialize_str("Six"),
			SuggestedScore::Seven => serializer.serialize_str("Seven"),
			SuggestedScore::Eight => serializer.serialize_str("Eight"),
			SuggestedScore::Nine => serializer.serialize_str("Nine"),
			SuggestedScore::Ten => serializer.serialize_str("Ten")
		}
	}
}

impl FromStr for SuggestedRating {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Rate" => Ok(Self::Rate),
			"Feature" => Ok(Self::Feature),
			"Epic" => Ok(Self::Epic),
			"Legendary" => Ok(Self::Legendary),
			"Mythic" => Ok(Self::Mythic),
			_ => Err(())
		}
	}
}

impl Serialize for SuggestedRating {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		match self {
			SuggestedRating::Rate => serializer.serialize_str("Rate"),
			SuggestedRating::Feature => serializer.serialize_str("Feature"),
			SuggestedRating::Epic => serializer.serialize_str("Epic"),
			SuggestedRating::Legendary => serializer.serialize_str("Legendary"),
			SuggestedRating::Mythic => serializer.serialize_str("Mythic")
		}
	}
}
