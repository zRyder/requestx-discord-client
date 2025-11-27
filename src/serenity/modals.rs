use serenity::all::{
	Component, CreateComponent, CreateLabel, CreateSelectMenu, CreateSelectMenuKind,
	CreateSelectMenuOption, LabelComponent, ModalInteraction,
};
use serenity::{
	all::{CreateModal, InputTextStyle},
	builder::CreateInputText,
};
use std::borrow::Cow;
use std::collections::HashMap;

pub fn get_init_gd_account_link_modal<'a>() -> CreateModal<'a> {
	let mut rows: Vec<CreateComponent> = Vec::new();
	rows.push(CreateComponent::Label(CreateLabel::input_text(
		"Geometry Dash Username",
		CreateInputText::new(InputTextStyle::Short, "gd-username")
			.placeholder("Ryder")
			.min_length(1)
			.max_length(30)
			.required(true),
	)));
	CreateModal::new(
		"init-gd-account-link-modal",
		"Link Your Geometry Dash Account",
	)
	.components(rows)
}

pub fn get_request_level_modal<'a>() -> CreateModal<'a> {
	let mut rows: Vec<CreateComponent> = Vec::new();
	rows.push(CreateComponent::Label(CreateLabel::input_text(
		"Level ID",
		CreateInputText::new(InputTextStyle::Short, "level-id")
			.placeholder("72308725")
			.min_length(1)
			.max_length(10)
			.required(true),
	)));
	rows.push(CreateComponent::Label(CreateLabel::select_menu(
		"Requested Rating",
		CreateSelectMenu::new(
			"request-rating",
			CreateSelectMenuKind::String {
				options: Cow::Owned(get_request_level_request_rating_options()),
			},
		)
		.min_values(1)
		.max_values(1),
	)));
	rows.push(CreateComponent::Label(CreateLabel::input_text(
		"YouTube Video",
		CreateInputText::new(InputTextStyle::Short, "video-link")
			.placeholder("https://youtu.be/lzMQWS9XvJo?si=IKnHdi0h_7K4lNSH")
			.required(true),
	)));
	rows.push(CreateComponent::Label(
		CreateLabel::select_menu(
			"Request feedback on this level request?",
			CreateSelectMenu::new(
				"request-feedback",
				CreateSelectMenuKind::String {
					options: Cow::Owned(get_yes_no_options()),
				},
			)
			.min_values(1)
			.max_values(1),
		)
		.description(
			"Select \"Yes\" if you would like to potentially receive feedback on this \
			level request.",
		),
	));
	rows.push(CreateComponent::Label(
		CreateLabel::select_menu(
			"Get notified when a request is reviewed?",
			CreateSelectMenu::new(
				"notify",
				CreateSelectMenuKind::String {
					options: Cow::Owned(get_yes_no_options()),
				},
			)
			.min_values(1)
			.max_values(1),
		)
		.description(
			"Select \"Yes\" if you would like to be pinged when your \
			 request if reviewed or sent",
		),
	));

	CreateModal::new("request-level-modal", "Request a Level").components(rows)
}

fn get_request_level_request_rating_options<'a>() -> Vec<CreateSelectMenuOption<'a>> {
	vec![
		CreateSelectMenuOption::new("Auto, 1 Star/Moon", "One"),
		CreateSelectMenuOption::new("Easy, 2 Stars/Moons", "Two"),
		CreateSelectMenuOption::new("Normal, 3 Stars/Moons", "Three"),
		CreateSelectMenuOption::new("Hard, 4 Stars/Moons", "Four"),
		CreateSelectMenuOption::new("Hard, 5 Stars/Moons", "Five"),
		CreateSelectMenuOption::new("Harder, 6 Stars/Moons", "Six"),
		CreateSelectMenuOption::new("Harder, 7 Stars/Moons", "Seven"),
		CreateSelectMenuOption::new("Insane, 8 Stars/Moons", "Eight"),
		CreateSelectMenuOption::new("Insane, 9 Stars/Moons", "Nine"),
		CreateSelectMenuOption::new("Demon, 10 Stars/Moons", "Ten"),
	]
}

fn get_yes_no_options<'a>() -> Vec<CreateSelectMenuOption<'a>> {
	vec![
		CreateSelectMenuOption::new("Yes", "true"),
		CreateSelectMenuOption::new("No", "false"),
	]
}

pub fn extract_modal_components(
	modal_interaction: &ModalInteraction,
	ids_to_search: Vec<String>,
) -> HashMap<String, String> {
	let mut components_map: HashMap<String, String> = HashMap::new();
	let components = &modal_interaction.data.components;
	for component in components {
		match component {
			Component::Label(label) => match &label.component {
				LabelComponent::SelectMenu(select_menu) => {
					if ids_to_search.contains(&select_menu.custom_id.to_string()) {
						components_map.insert(
							select_menu.custom_id.to_string(),
							select_menu.values.get(0).unwrap().to_string(),
						);
					}
				}
				LabelComponent::InputText(input_text) => {
					if let Some(input_text_value) = &input_text.value {
						if ids_to_search.contains(&input_text.custom_id.to_string()) {
							components_map.insert(
								input_text.custom_id.to_string(),
								input_text_value.to_string(),
							);
						}
					}
				}
				_ => {}
			},
			_ => {}
		}
	}
	components_map
}
