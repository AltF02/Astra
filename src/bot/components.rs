use crate::models::launch::LaunchID;
use serenity::builder::{CreateActionRow, CreateButton, CreateComponents};
use serenity::model::prelude::message_component::ButtonStyle;

pub struct RemindComponent<'a> {
    launch_id: &'a LaunchID,
}

impl<'a> RemindComponent<'a> {
    pub fn new(launch_id: &'a LaunchID) -> RemindComponent {
        Self { launch_id }
    }

    fn button(&self) -> CreateButton {
        let mut b = CreateButton::default();
        b.custom_id(self.launch_id);
        b.label("Remind Me");
        b.style(ButtonStyle::Primary);
        b
    }

    fn action_row(&self) -> CreateActionRow {
        let mut ar = CreateActionRow::default();
        ar.add_button(self.button());
        ar
    }
}

pub fn create_basic_components() -> CreateComponents {
    CreateComponents::default()
}

pub fn create_launch_components(launch_id: &LaunchID) -> CreateComponents {
    let mut c = create_basic_components();

    let rc = RemindComponent::new(launch_id);
    c.add_action_row(rc.action_row());
    c
}
