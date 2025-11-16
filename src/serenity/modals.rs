use serenity::all::{CreateActionRow, CreateModal, InputTextStyle};
use serenity::builder::CreateInputText;

pub fn get_init_gd_account_link_modal() -> CreateModal {
    let mut rows: Vec<CreateActionRow> = Vec::new();
    rows.push(
        CreateActionRow::InputText(
            CreateInputText::new(
                InputTextStyle::Short,
                "Geometry Dash Username",
                "gd-username"
            )
                .placeholder("Ryder")
                .min_length(1)
                .min_length(30)
                .required(true)
        )
    );
    CreateModal::new(
        "init-gd-account-link-modal",
        "Link Your Geometry Dash Account"
    )
        .components(rows)
}