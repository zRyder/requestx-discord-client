use serde::{Serialize, Serializer};

pub enum LevelType {
	Classic,
	Platformer
}

impl Serialize for LevelType {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		match self {
			LevelType::Classic => serializer.serialize_str("Classic"),
			LevelType::Platformer => serializer.serialize_str("Platformer")
		}
	}
}
