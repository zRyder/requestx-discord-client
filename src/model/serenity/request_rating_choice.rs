#[derive(Debug, poise::ChoiceParameter)]
pub enum RequestRatingChoice {
	#[name = "Auto, 1 Star/Moon"]
	One,
	#[name = "Easy, 2 Stars/Moons"]
	Two,
	#[name = "Normal, 3 Stars/Moons"]
	Three,
	#[name = "Hard, 4 Stars/Moons"]
	Four,
	#[name = "Hard, 5 Stars/Moons"]
	Five,
	#[name = "Harder, 6 Stars/Moons"]
	Six,
	#[name = "Harder, 7 Stars/Moons"]
	Seven,
	#[name = "Insane, 8 Stars/Moons"]
	Eight,
	#[name = "Insane, 9 Stars/Moons"]
	Nine,
	#[name = "Demon, 10 Stars/Moons"]
	Ten
}
