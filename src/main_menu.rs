use bevy::{app::AppExit, prelude::*};

use crate::{button::ColoredButton, fonts::{PaintFont, RobotoFont}, GameState};

#[derive(Component)]
struct MainMenuUi;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct ExitButton;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(load_ui))
			.add_system_set(
				SystemSet::on_update(GameState::MainMenu)
					.with_system(play_button)
					.with_system(exit_button)
					.with_system(settings_button),
			)
			.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(drop_ui));
	}
}

fn load_ui(mut commands: Commands, paint_font: Res<PaintFont>, roboto_font: Res<RobotoFont>) {
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
		.insert(MainMenuUi)
		.insert(Name::new("Ui"))
		.with_children(|parent| {
			parent
				.spawn_bundle(
					TextBundle::from_section(
						"Crackhead Adventures",
						TextStyle {
							font: paint_font.0.clone(),
							font_size: 132.0,
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
				.spawn_bundle(NodeBundle {
					style: Style {
						#[cfg(not(target_arch="wasm32"))]
						size: Size::new(Val::Percent(50.0), Val::Px(200.0)),
						#[cfg(target_arch="wasm32")]
						size: Size::new(Val::Percent(50.0), Val::Px(135.0)),
						justify_content: JustifyContent::SpaceBetween,
						flex_direction: FlexDirection::ColumnReverse,
						align_items: AlignItems::Center,
						margin: UiRect::new(
							Val::Px(0.0),
							Val::Px(0.0),
							Val::Px(100.0),
							Val::Px(0.0),
						),
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
								size: Size::new(Val::Px(300.0), Val::Px(50.0)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..Default::default()
							},
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("PlayButton"))
						.insert(ColoredButton::default())
						.insert(PlayButton)
						.with_children(|parent| {
							parent.spawn_bundle(TextBundle::from_section(
								"Play",
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
								size: Size::new(Val::Px(300.0), Val::Px(50.0)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..Default::default()
							},
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("SettingsButton"))
						.insert(ColoredButton::default())
						.insert(SettingsButton)
						.with_children(|parent| {
							parent.spawn_bundle(TextBundle::from_section(
								"Settings",
								TextStyle {
									font: roboto_font.0.clone(),
									font_size: 32.0,
									color: Color::BLACK,
								},
							));
						});
					#[cfg(not(target_arch="wasm32"))]
					parent
						.spawn_bundle(ButtonBundle {
							style: Style {
								size: Size::new(Val::Px(300.0), Val::Px(50.0)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..Default::default()
							},
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("ExitButton"))
						.insert(ColoredButton::default())
						.insert(ExitButton)
						.with_children(|parent| {
							parent.spawn_bundle(TextBundle::from_section(
								"Exit",
								TextStyle {
									font: roboto_font.0.clone(),
									font_size: 32.0,
									color: Color::BLACK,
								},
							));
						});
				});

			parent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(100.0), Val::Auto),
						justify_content: JustifyContent::FlexEnd,
						align_items: AlignItems::FlexStart,
						align_self: AlignSelf::FlexEnd,
						// this needs a fix quick
						#[cfg(not(target_arch="wasm32"))]
						position: UiRect::new(
							Val::Px(0.0),
							Val::Px(0.0),
							Val::Px(120.0),
							Val::Px(0.0),
						),
						#[cfg(target_arch="wasm32")]
						position: UiRect::new(
							Val::Px(0.0),
							Val::Px(0.0),
							Val::Px(160.0),
							Val::Px(0.0),
						),
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				})
				.insert(Name::new("VersionContainer"))
				.with_children(|parent| {
				parent
					.spawn_bundle(
						TextBundle::from_section(
							format!("v{}", env!("CARGO_PKG_VERSION")),
							TextStyle {
								font: paint_font.0.clone(),
								font_size: 16.0,
								color: Color::WHITE,
							},
						)
						.with_style(Style {
							margin: UiRect::all(Val::Px(5.0)),
							..default()
						}),
					)
					.insert(Name::new("Version"));
				});


		});
}

fn drop_ui(mut commands: Commands, ui: Query<Entity, With<MainMenuUi>>) {
	let ui = ui.single();
	commands.entity(ui).despawn_recursive();
}

fn play_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
	mut state: ResMut<State<GameState>>,
) {
	for interaction in &mut interaction_query {
		#[allow(clippy::collapsible_if)]
		if *interaction == Interaction::Clicked {
			if state.set(GameState::Game).is_err() {}
		}
	}
}

fn settings_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
	mut state: ResMut<State<GameState>>,
) {
	for interaction in &mut interaction_query {
		#[allow(clippy::collapsible_if)]
		if *interaction == Interaction::Clicked {
			if state.set(GameState::Settings).is_err() {}
		}
	}
}

fn exit_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
	mut exit: EventWriter<AppExit>,
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			exit.send(AppExit);
		}
	}
}
