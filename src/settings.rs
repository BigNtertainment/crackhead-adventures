use bevy::prelude::*;

use crate::{
	button::ColoredButton,
	fonts::{PaintFont, RobotoFont},
	GameState,
};

#[derive(Component)]
struct SettingsUi;

#[derive(Component)]
struct MainMenuButton;

#[derive(Component)]
struct MusicVolumeAmount;

#[derive(Component)]
struct SfxVolumeAmount;

#[derive(Component)]
struct SubSfxButton;

#[derive(Component)]
struct AddSfxButton;

#[derive(Component)]
struct SubMusicButton;

#[derive(Component)]
struct AddMusicButton;

pub struct SettingsPlugin;

pub struct Settings {
	pub sfx_volume: f64,
	pub music_volume: f64,
}

impl Plugin for SettingsPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Settings {
			sfx_volume: 1.0,
			music_volume: 1.0,
		})
		.add_system_set(SystemSet::on_enter(GameState::Settings).with_system(load_ui))
		.add_system_set(SystemSet::on_update(GameState::Settings)
			.with_system(update_ui)
			.with_system(main_menu_button)
			.with_system(sub_music_button)
			.with_system(add_music_button)
			.with_system(sub_sfx_button)
			.with_system(add_sfx_button)
		)
		.add_system_set(SystemSet::on_exit(GameState::Settings).with_system(drop_ui));
	}
}

fn load_ui(
	mut commands: Commands,
	roboto_font: Res<RobotoFont>,
	paint_font: Res<PaintFont>,
	settings: Res<Settings>,
) {
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				padding: UiRect::new(Val::Px(100.0), Val::Px(0.0), Val::Px(100.0), Val::Px(0.0)),
				justify_content: JustifyContent::FlexStart,
				align_items: AlignItems::FlexStart,
				flex_direction: FlexDirection::ColumnReverse,
				..Default::default()
			},
			color: UiColor(Color::BLACK),
			..Default::default()
		})
		.insert(SettingsUi)
		.insert(Name::new("Ui"))
		.with_children(|parent| {
			parent
				.spawn_bundle(ButtonBundle {
					style: Style {
						size: Size::new(Val::Px(300.0), Val::Px(50.0)),
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
					parent.spawn_bundle(TextBundle::from_section(
						"Main Menu",
						TextStyle {
							font: roboto_font.0.clone(),
							font_size: 32.0,
							color: Color::BLACK,
						},
					));
				});

			parent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(50.0), Val::Px(50.0)),
						justify_content: JustifyContent::SpaceBetween,
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
				.insert(Name::new("MusicContainer"))
				.with_children(|parent| {
					parent
						.spawn_bundle(
							TextBundle::from_section(
								format!("Music: "),
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
						.insert(Name::new("MusicVolumeLabel"));

					parent
						.spawn_bundle(NodeBundle {
							style: Style {
								size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
								..Default::default()
							},
							color: Color::NONE.into(),
							..Default::default()
						})
						.insert(Name::new("MusicButtonsContainer"))
						.with_children(|parent| {
							parent
								.spawn_bundle(ButtonBundle {
									style: Style {
										size: Size::new(Val::Px(60.0), Val::Percent(100.0)),
										justify_content: JustifyContent::Center,
										align_items: AlignItems::Center,
										..Default::default()
									},
									button: Button,
									color: Color::RED.into(),
									..Default::default()
								})
								.insert(Name::new("SubMusicButton"))
								.insert(SubMusicButton)
								.insert(ColoredButton::default())
								.with_children(|parent| {
									parent.spawn_bundle(TextBundle::from_section(
										"-",
										TextStyle {
											font: roboto_font.0.clone(),
											font_size: 32.0,
											color: Color::BLACK,
										},
									));
								});

							parent
								.spawn_bundle(
									TextBundle::from_section(
										format!("{:3.0}", settings.music_volume * 100.0),
										TextStyle {
											font: paint_font.0.clone(),
											font_size: 32.0,
											color: Color::WHITE,
										},
									)
									.with_style(Style {
										size: Size::new(Val::Px(100.0), Val::Percent(100.0)),
										margin: UiRect::all(Val::Px(5.0)),
										..default()
									}),
								)
								.insert(Name::new("MusicVolumeAmount"))
								.insert(MusicVolumeAmount);

							parent
								.spawn_bundle(ButtonBundle {
									style: Style {
										size: Size::new(Val::Px(60.0), Val::Percent(100.0)),
										justify_content: JustifyContent::Center,
										align_items: AlignItems::Center,
										..Default::default()
									},
									button: Button,
									color: Color::RED.into(),
									..Default::default()
								})
								.insert(Name::new("AddMusicButton"))
								.insert(AddMusicButton)
								.insert(ColoredButton::default())
								.with_children(|parent| {
									parent.spawn_bundle(TextBundle::from_section(
										"+",
										TextStyle {
											font: roboto_font.0.clone(),
											font_size: 32.0,
											color: Color::BLACK,
										},
									));
								});
						});
				});

			parent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(50.0), Val::Px(50.0)),
						justify_content: JustifyContent::SpaceBetween,
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				})
				.insert(Name::new("SfxContainer"))
				.with_children(|parent| {
					parent
						.spawn_bundle(
							TextBundle::from_section(
								format!("SFX: "),
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
						.insert(Name::new("SfxVolumeLabel"));

					parent
						.spawn_bundle(NodeBundle {
							style: Style {
								size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
								..Default::default()
							},
							color: Color::NONE.into(),
							..Default::default()
						})
						.insert(Name::new("SfxButtonsContainer"))
						.with_children(|parent| {

							parent
								.spawn_bundle(ButtonBundle {
									style: Style {
										size: Size::new(Val::Px(60.0), Val::Percent(100.0)),
										justify_content: JustifyContent::Center,
										align_items: AlignItems::Center,
										..Default::default()
									},
									button: Button,
									color: Color::RED.into(),
									..Default::default()
								})
								.insert(Name::new("SubSfxButton"))
								.insert(SubSfxButton)
								.insert(ColoredButton::default())
								.with_children(|parent| {
									parent.spawn_bundle(TextBundle::from_section(
										"-",
										TextStyle {
											font: roboto_font.0.clone(),
											font_size: 32.0,
											color: Color::BLACK,
										},
									));
								});

							parent
								.spawn_bundle(
									TextBundle::from_section(
										format!("{:3.0}", settings.sfx_volume * 100.0),
										TextStyle {
											font: paint_font.0.clone(),
											font_size: 32.0,
											color: Color::WHITE,
										},
									)
									.with_style(Style {
										size: Size::new(Val::Px(100.0), Val::Percent(100.0)),
										margin: UiRect::all(Val::Px(5.0)),
										..default()
									}),
								)
								.insert(Name::new("SfxVolumeAmount"))
								.insert(SfxVolumeAmount);

							parent
								.spawn_bundle(ButtonBundle {
									style: Style {
										size: Size::new(Val::Px(60.0), Val::Percent(100.0)),
										justify_content: JustifyContent::Center,
										align_items: AlignItems::Center,
										..Default::default()
									},
									button: Button,
									color: Color::RED.into(),
									..Default::default()
								})
								.insert(Name::new("AddSfxButton"))
								.insert(AddSfxButton)
								.insert(ColoredButton::default())
								.with_children(|parent| {
									parent.spawn_bundle(TextBundle::from_section(
										"+",
										TextStyle {
											font: roboto_font.0.clone(),
											font_size: 32.0,
											color: Color::BLACK,
										},
									));
								});
						});
				});
		});
}

fn update_ui(
	settings: Res<Settings>,
	mut music_volume_query: Query<
        &mut Text,
        (
            With<MusicVolumeAmount>,
            Without<SfxVolumeAmount>,
        ),
    >,
	mut sfx_volume_query: Query<
        &mut Text,
        (
            With<SfxVolumeAmount>,
            Without<MusicVolumeAmount>,
        ),
    >,
) {
	let mut music_volume = music_volume_query.single_mut();
	music_volume.sections[0].value = format!("{:3.0}", settings.music_volume * 100.0);

	let mut sfx_volume = sfx_volume_query.single_mut();
	sfx_volume.sections[0].value = format!("{:3.0}", settings.sfx_volume * 100.0);
}

fn drop_ui(mut commands: Commands, ui: Query<Entity, With<SettingsUi>>) {
	let ui = ui.single();
	commands.entity(ui).despawn_recursive();
}

fn sub_music_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SubMusicButton>)>,
	mut settings: ResMut<Settings>
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			settings.music_volume = (settings.music_volume - 0.05).clamp(0.0, 1.0);	
		}
	}
}

fn add_music_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<AddMusicButton>)>,
	mut settings: ResMut<Settings>
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			settings.music_volume = (settings.music_volume + 0.05).clamp(0.0, 1.0);	
		}
	}
	
}

fn sub_sfx_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SubSfxButton>)>,
	mut settings: ResMut<Settings>
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			settings.sfx_volume = (settings.sfx_volume - 0.05).clamp(0.0, 1.0);	
		}
	}
}

fn add_sfx_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<AddSfxButton>)>,
	mut settings: ResMut<Settings>
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			settings.sfx_volume = (settings.sfx_volume + 0.05).clamp(0.0, 1.0);	
		}
	}
	
}

fn main_menu_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
	mut state: ResMut<State<GameState>>,
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			state
				.set(GameState::MainMenu)
				.expect("Failed to change state!");
		}
	}
}
