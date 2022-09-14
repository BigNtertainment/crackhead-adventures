use bevy::prelude::*;

use crate::{GameState, fonts::{PaintFont, RobotoFont}, button::ColoredButton, stats::Stats};

#[derive(Component)]
struct GameOverUi;

#[derive(Component)]
struct RetryButton;

#[derive(Component)]
struct StatsButton;

#[derive(Component)]
struct MainMenuButton;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system_set(
				SystemSet::on_enter(GameState::GameOver)
					.with_system(load_ui)
			)
			.add_system_set(
				SystemSet::on_update(GameState::GameOver)
					.with_system(retry_button)
					.with_system(main_menu_button)
					.with_system(stats_button)
			)
			.add_system_set(
				SystemSet::on_exit(GameState::GameOver)
					.with_system(drop_ui)
			);
	}
}

fn load_ui(mut commands: Commands, paint_font: Res<PaintFont>, roboto_font: Res<RobotoFont>, stats: Res<Stats>) {
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
							font: paint_font.0.clone(),
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
							font: paint_font.0.clone(),
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
				.spawn_bundle(
					TextBundle::from_section(
						format!("You died in {:.2}s", stats.timer.elapsed_secs()),
						TextStyle {
							font: paint_font.0.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Time"));

			parent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(50.0), Val::Px(250.0)),
						justify_content: JustifyContent::SpaceBetween,
						flex_direction: FlexDirection::ColumnReverse,
						align_items: AlignItems::Center,
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
								size: Size::new(Val::Px(300.0), Val::Percent(20.0)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..Default::default()
							},
							button: Button,
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("RetryButton"))
						.insert(ColoredButton::default())
						.insert(RetryButton)
						.with_children(|parent| {
							parent
								.spawn_bundle(TextBundle::from_section(
									"Retry",
									TextStyle {
										font: roboto_font.0.clone(),
										font_size: 32.0,
										color: Color::BLACK
									}
								));
						});

					parent
						.spawn_bundle(ButtonBundle {
							style: Style {
								size: Size::new(Val::Px(300.0), Val::Percent(20.0)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..Default::default()
							},
							button: Button,
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("StatsButton"))
						.insert(StatsButton)
						.insert(ColoredButton::default())
						.with_children(|parent| {
							parent.spawn_bundle(TextBundle::from_section(
								"Stats",
								TextStyle {
									font: roboto_font.0.clone(),
									font_size: 32.0,
									color: Color::BLACK,
								},
							));
						});

					parent
						.spawn_bundle(ButtonBundle {
							style: Style {
								size: Size::new(Val::Px(300.0), Val::Percent(20.0)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..Default::default()
							},
							button: Button,
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("MainMenuButton"))
						.insert(MainMenuButton)
						.insert(ColoredButton::default())
						.with_children(|parent| {
							parent
								.spawn_bundle(TextBundle::from_section(
									"Main Menu",
									TextStyle {
										font: roboto_font.0.clone(),
										font_size: 32.0,
										color: Color::BLACK
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

fn stats_button(
	mut interaction_query: Query<
		&Interaction,
		(Changed<Interaction>, With<StatsButton>)
	>,
	mut state: ResMut<State<GameState>>
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			state.set(GameState::Stats).expect("Failed to change state!");
		}
	}
}

fn main_menu_button(
	mut interaction_query: Query<
		&Interaction,
		(Changed<Interaction>, With<MainMenuButton>)
	>,
	mut state: ResMut<State<GameState>>
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			state.set(GameState::MainMenu).expect("Failed to change state!");
		}
	}
}