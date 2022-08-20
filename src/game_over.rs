use bevy::prelude::*;

use crate::GameState;

const BUTTON_COLOR: Color = Color::rgb(219.0 / 255.0, 33.0 / 255.0, 20.0 / 255.0);
const HOVERED_BUTTON_COLOR: Color = Color::rgb(214.0 / 255.0, 60.0 / 255.0, 49.0 / 255.0);
const PRESSED_BUTTON_COLOR: Color = Color::rgb(219.0 / 255.0, 76.0 / 255.0, 66.0 / 255.0);

#[derive(Component)]
struct GameOverUi;

#[derive(Component)]
struct RetryButton;

#[derive(Component)]
struct MainMenuButton;

pub struct PaintFont(Handle<Font>);

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(load_font)
			.add_system_set(
				SystemSet::on_enter(GameState::GameOver)
					.with_system(load_ui)
			)
			.add_system_set(
				SystemSet::on_update(GameState::GameOver)
					.with_system(update_button_colors)
					.with_system(retry_button)
			)
			.add_system_set(
				SystemSet::on_exit(GameState::GameOver)
					.with_system(drop_ui)
			);
	}
}

fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(
		PaintFont(asset_server.load("floyd.ttf"))
	);
}

fn load_ui(mut commands: Commands, font: Res<PaintFont>) {
	let font = &font.0;

	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				flex_direction: FlexDirection::ColumnReverse,
				..Default::default()
			},
			color: UiColor(Color::BLACK),
			..Default::default()
		})
		.insert(GameOverUi)
		.insert(Name::new("Ui"))
		.with_children(|parent| {
			parent
				.spawn_bundle(
					TextBundle::from_section(
						"Game Over",
						TextStyle {
							font: font.clone(),
							font_size: 152.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Title"));

			parent
				.spawn_bundle(
					TextBundle::from_section(
						"Guess the drugs were bad for you after all",
						TextStyle {
							font: font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Subtitle"));

			parent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(50.0), Val::Px(50.0)),
						justify_content: JustifyContent::SpaceBetween,
						margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(100.0), Val::Px(0.0)),
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				})
				.insert(Name::new("ButtonsContainer"))
				.with_children(|parent| {
					parent
						.spawn_bundle(ButtonBundle {
							style: Style {
								size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..Default::default()
							},
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("RetryButton"))
						.insert(RetryButton)
						.with_children(|parent| {
							parent
								.spawn_bundle(TextBundle::from_section(
									"Retry",
									TextStyle {
										font: font.clone(),
										font_size: 32.0,
										color: Color::BLACK.into()
									}
								));
						});

					parent
						.spawn_bundle(ButtonBundle {
							style: Style {
								size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..Default::default()
							},
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("MainMenuButton"))
						.insert(MainMenuButton)
						.with_children(|parent| {
							parent
								.spawn_bundle(TextBundle::from_section(
									"Main Menu",
									TextStyle {
										font: font.clone(),
										font_size: 32.0,
										color: Color::BLACK.into()
									}
								));
						});
				});
		});
}

fn drop_ui(mut commands: Commands, ui: Query<Entity, With<GameOverUi>>) {
	let ui = ui.single();
	commands.entity(ui).despawn_recursive();
}

fn retry_button(
	mut interaction_query: Query<
		&Interaction,
		(Changed<Interaction>, With<RetryButton>)
	>,
	mut state: ResMut<State<GameState>>
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			state.set(GameState::Game).expect("Failed to change state!");
		}
	}
}

fn update_button_colors(
	mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >
) {
	for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR.into();
            }
        }
    }
}