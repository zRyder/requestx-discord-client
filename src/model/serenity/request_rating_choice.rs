use crate::model::request_score::RequestRating;

pub enum RequestRatingChoice {
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

impl Into<RequestRating> for RequestRatingChoice {
	fn into(self) -> RequestRating {
		match self {
			RequestRatingChoice::One => RequestRating::One,
			RequestRatingChoice::Two => RequestRating::Two,
			RequestRatingChoice::Three => RequestRating::Three,
			RequestRatingChoice::Four => RequestRating::Four,
			RequestRatingChoice::Five => RequestRating::Five,
			RequestRatingChoice::Six => RequestRating::Six,
			RequestRatingChoice::Seven => RequestRating::Seven,
			RequestRatingChoice::Eight => RequestRating::Eight,
			RequestRatingChoice::Nine => RequestRating::Nine,
			RequestRatingChoice::Ten => RequestRating::Ten
		}
	}
}
