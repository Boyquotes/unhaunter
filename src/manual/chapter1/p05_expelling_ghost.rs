use crate::root::GameAssets;
use bevy::prelude::*;

pub fn draw_expelling_ghost_page(parent: &mut ChildBuilder, handles: &GameAssets) {
    parent.spawn(
        TextBundle::from_section(
            "Once you've crafted the repellent, confront the ghost and use it to banish it. Return to your truck and click 'End Mission' to complete the investigation.",
            TextStyle {
                font: handles.fonts.londrina.w300_light.clone(),
                font_size: 38.0,
                color: Color::WHITE,
            },
        ),
    );
}
