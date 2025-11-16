use std::collections::HashMap;
use serenity::{
	all::{CreateActionRow, CreateModal, InputTextStyle},
	builder::CreateInputText
};
use serenity::all::{ActionRowComponent, ModalInteraction};

pub fn get_init_gd_account_link_modal() -> CreateModal {
	let mut rows: Vec<CreateActionRow> = Vec::new();
	rows.push(CreateActionRow::InputText(
		CreateInputText::new(
			InputTextStyle::Short,
			"Geometry Dash Username",
			"gd-username"
		)
		.placeholder("Ryder")
		.min_length(1)
		.max_length(30)
		.required(true)
	));
	CreateModal::new(
		"init-gd-account-link-modal",
		"Link Your Geometry Dash Account"
	)
	.components(rows)
}

pub fn extract_modal_components(
	modal_interaction: &ModalInteraction,
	ids_to_search: Vec<String>
) ->  HashMap<&String, &Option<String>> {
	let mut components_map: HashMap<&String, &Option<String>> = HashMap::new();
	let components = &modal_interaction.data.components;
	for component in components {
		if let Some(action_row) = component.components.get(0) {
			match action_row {
				ActionRowComponent::Button(_) => {

				}
				ActionRowComponent::SelectMenu(_) => {
					// if let Some(id) = select_menu.custom_id {
					//     if ids_to_search.contains(&id) {
					//
					//         components_map.insert(id, select_menu.);
					//     }
					// }
				}
				ActionRowComponent::InputText(input_text) => {
					if ids_to_search.contains(&input_text.custom_id) {
						components_map.insert(&input_text.custom_id, &input_text.value);
					}
				}
				_ => {}
			}
		}
	}
	components_map
}
