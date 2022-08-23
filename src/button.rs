use bevy::prelude::*;

#[derive(Component)]
pub struct ColoredButton {
	pub color: Color,
	pub hovered_color: Color,
	pub pressed_color: Color,
}

impl Default for ColoredButton {
	fn default() -> Self {
		Self {
			color: Color::rgb(219.0 / 255.0, 33.0 / 255.0, 20.0 / 255.0),
			hovered_color: Color::rgb(214.0 / 255.0, 60.0 / 255.0, 49.0 / 255.0),
			pressed_color: Color::rgb(219.0 / 255.0, 76.0 / 255.0, 66.0 / 255.0),
		}
	}
}

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(update_button_colors);
	}
}

fn update_button_colors(
	mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ColoredButton),
        (Changed<Interaction>, With<ColoredButton>),
    >
) {
	for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = button.pressed_color.into();
            }
            Interaction::Hovered => {
                *color = button.hovered_color.into();
            }
            Interaction::None => {
                *color = button.color.into();
            }
        }
    }
}