use bevy::{prelude::*, time::Stopwatch};

use crate::{GameState, fonts::{PaintFont, RobotoFont}, button::ColoredButton};

#[derive(Component)]
struct StatsUi;
#[derive(Component)]

struct MainMenuButton;

pub struct StatsPlugin;

#[derive(Debug)]
pub struct Stats{
    pub timer: Stopwatch,
    pub enemies_killed: u16,
    pub small_powerup_used: u16,
    pub small_powerup_collected: u16,
    pub big_powerup_used: u16,
    pub big_powerup_crafted: u16,
    pub damage_taken: f32,
    pub shot_fired: u16,
    pub shot_accuracy: f32,
}

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Stats{
            timer: Stopwatch::new(),
            enemies_killed: 0,
            small_powerup_used: 0,
            small_powerup_collected: 0,
            big_powerup_used: 0,
            big_powerup_crafted: 0,
            damage_taken: 0.0,
            shot_fired: 0,
            shot_accuracy:0.0,
        })
           .add_system_set(SystemSet::on_enter(GameState::Game).with_system(reset_stats))
           .add_system_set(SystemSet::on_exit(GameState::Game).with_system(calculate_stats))
           .add_system_set(SystemSet::on_update(GameState::Game).with_system(update_stats))
           .add_system_set(SystemSet::on_enter(GameState::Stats).with_system(load_ui))
           .add_system_set(SystemSet::on_update(GameState::Stats).with_system(main_menu_button))
           .add_system_set(SystemSet::on_exit(GameState::Stats).with_system(drop_ui));
        
        
    }
}


fn reset_stats(mut stats: ResMut<Stats>) {
    stats.timer.reset();
    stats.timer.unpause();

    stats.enemies_killed = 0; //todo
    stats.small_powerup_used = 0;
    stats.small_powerup_collected = 0;
    stats.big_powerup_used = 0;
    stats.big_powerup_crafted = 0;
    stats.damage_taken = 0.0;
    stats.shot_fired = 0;
    stats.shot_accuracy = 0.0; 
}

fn update_stats(mut stats: ResMut<Stats>, time: Res<Time>) {
    stats.timer.tick(time.delta());
}
 
fn calculate_stats(mut stats: ResMut<Stats>) {
    stats.timer.pause();

    if stats.shot_fired != 0 {
        stats.shot_accuracy = (stats.enemies_killed / stats.shot_fired) as f32 * 100.0; 
    }

    println!("{:?}", stats);
}

fn load_ui(mut commands: Commands, paint_font: Res<PaintFont>, roboto_font: Res<RobotoFont>, stats: Res<Stats>) {
    let paint_font = &paint_font.0;
    let roboto_font = &roboto_font.0;

	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::FlexStart,
				align_items: AlignItems::FlexStart,
				flex_direction: FlexDirection::ColumnReverse,
                padding: UiRect::new(Val::Px(100.0), Val::Px(0.0), Val::Px(-10.0), Val::Px(0.0)),
				..Default::default()
			},
			color: UiColor(Color::BLACK),
			..Default::default()
		})
		.insert(StatsUi)
		.insert(Name::new("Ui"))
        .with_children(|parent| {
            parent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(50.0), Val::Px(50.0)),
						justify_content: JustifyContent::FlexStart,
						flex_direction: FlexDirection::ColumnReverse,
						align_items: AlignItems::FlexStart,
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
										font: roboto_font.clone(),
										font_size: 32.0,
										color: Color::BLACK
									}
								));
                    });
                });

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::ColumnReverse,
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(100.0), Val::Px(0.0)),
                    ..Default::default()
                },
                color: UiColor(Color::BLACK),
                ..Default::default()
            })
            .insert(Name::new("Stats Container"))
            .with_children(|parent| {
                parent
				.spawn_bundle(
					TextBundle::from_section(
						format!("Level finished in: {:.2}s", stats.timer.elapsed_secs()),
						TextStyle {
							font: paint_font.clone(),
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
				.spawn_bundle(
					TextBundle::from_section(
						format!("Enemies killed: {}", stats.enemies_killed),
						TextStyle {
							font: paint_font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Killed Enemies"));

                parent
				.spawn_bundle(
					TextBundle::from_section(
						format!("Damage taken: {:.2}", stats.damage_taken),
						TextStyle {
							font: paint_font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Damage Taken"));

                parent
				.spawn_bundle(
					TextBundle::from_section(
						format!("Shots Taken: {}", stats.shot_fired),
						TextStyle {
							font: paint_font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Shots Taken"));

                parent
				.spawn_bundle(
					TextBundle::from_section(
						format!("Shot Accuracy: {:.2}%", stats.shot_accuracy),
						TextStyle {
							font: paint_font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Shot Accuracy"));

                parent
				.spawn_bundle(
					TextBundle::from_section(
						format!("Cocaine snorted: {}", stats.small_powerup_used),
						TextStyle {
							font: paint_font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Small Power Ups Used"));

                parent
				.spawn_bundle(
					TextBundle::from_section(
						format!("Cocaine stolen: {}", stats.small_powerup_collected),
						TextStyle {
							font: paint_font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Small Power Ups Collected"));

                parent
				.spawn_bundle(
					TextBundle::from_section(
						format!("Fun Dust administered: {}", stats.big_powerup_used),
						TextStyle {
							font: paint_font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Big Power Ups Used"));

                parent
				.spawn_bundle(
					TextBundle::from_section(
						format!("Fun Dust crafted: {}", stats.big_powerup_crafted),
						TextStyle {
							font: paint_font.clone(),
							font_size: 32.0,
							color: Color::WHITE,
						},
					)
					.with_style(Style {
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					}),
				)
				.insert(Name::new("Big Power Ups Crafted"));
            });
        });
}

fn drop_ui(mut commands: Commands, ui: Query<Entity, With<StatsUi>>) {
	let ui = ui.single();
	commands.entity(ui).despawn_recursive();
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