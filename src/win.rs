use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin}};

use crate::{
	button::ColoredButton,
	fonts::{PaintFont, RobotoFont},
	tilemap::Tile,
	GameState, level_timer::LevelTimer,
};

#[derive(Component, Default)]
pub struct Win;

#[derive(Component)]
struct WinUi;

#[derive(Component)]
struct PlayAgainButton;

#[derive(Component)]
struct MainMenuButton;

pub struct WinPlugin;

impl Plugin for WinPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(SystemSet::on_enter(GameState::Win).with_system(load_ui))
		.add_plugin(Material2dPlugin::<WinMaterial>::default())
			.add_system_set(
				SystemSet::on_update(GameState::Win)
				
					.with_system(update_win_material)
					.with_system(play_again_button)
					.with_system(main_menu_button),
			)
			.add_system_set(SystemSet::on_exit(GameState::Win).with_system(drop_ui));
	}
}

#[derive(Bundle, Default)]
pub struct WinBundle {
	#[bundle]
	sprite_budle: SpriteBundle,
	win: Win,
}

impl Tile for WinBundle {
	fn spawn(position: Vec2, texture: Handle<Image>, flip_x: bool, flip_y: bool) -> Self {
		WinBundle {
			sprite_budle: SpriteBundle {
				texture,
				transform: Transform::from_translation(position.extend(30.0)),
				sprite: Sprite {
					flip_x,
					flip_y,
					..Default::default()
				},
				..Default::default()
			},
			..Default::default()
		}
	}
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "2a387ac3-1602-4d2c-aa88-1acef30a287a"]
pub struct WinMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	pub source_image: Handle<Image>,
	#[uniform(2)]
	pub time: u32,
}

impl Material2d for WinMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/win.wgsl".into()
	}
}

fn update_win_material(win: Query<(&Handle<WinMaterial>, &Handle<Image>)>, time: Res<Time>, mut win_materials: ResMut<Assets<WinMaterial>>) {
	for (win_material, texture) in win.iter() {
		let mut win_material = win_materials.get_mut(win_material).unwrap();

		win_material.source_image = texture.clone();
		win_material.time = (time.seconds_since_startup() * 1000.0).floor() as u32;
	}
}

fn load_ui(mut commands: Commands, paint_font: Res<PaintFont>, roboto_font: Res<RobotoFont>, timer: Res<LevelTimer>) {
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
		.insert(WinUi)
		.insert(Name::new("Ui"))
		.with_children(|parent| {
			parent
				.spawn_bundle(
					TextBundle::from_section(
						"You win",
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
						"Thanks for playing!",
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
						format!("You got the spice in {:.2}s", timer.elapsed_secs()),
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
							button: Button,
							color: Color::RED.into(),
							..Default::default()
						})
						.insert(Name::new("PlayAgainButton"))
						.insert(ColoredButton::default())
						.insert(PlayAgainButton)
						.with_children(|parent| {
							parent.spawn_bundle(TextBundle::from_section(
								"Play again",
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
								size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
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
				});
		});
}

fn drop_ui(mut commands: Commands, ui: Query<Entity, With<WinUi>>) {
	let ui = ui.single();
	commands.entity(ui).despawn_recursive();
}

fn play_again_button(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayAgainButton>)>,
	mut state: ResMut<State<GameState>>,
) {
	for interaction in &mut interaction_query {
		if *interaction == Interaction::Clicked {
			state.set(GameState::Game).expect("Failed to change state!");
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
