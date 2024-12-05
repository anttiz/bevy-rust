use bevy::prelude::*;
use crate::game::level_config::get_current_level;

pub fn setup_level_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: format!("Level: {}", get_current_level()),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                    },
                },
            ],
            justify: JustifyText::Center,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Level Number UI"));
}

pub fn update_level_ui(mut query: Query<&mut Text, With<Name>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Level: {}", get_current_level());
    }
}