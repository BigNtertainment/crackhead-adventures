use bevy::prelude::*;

use crate::unit::{Health, Inventory};

use super::{EffectData, Player};

#[derive(Component)]
pub struct PlayerUi;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct SmallPowerUpCounterNumber;

#[derive(Component)]
pub struct BigPowerUpCounterNumber;

#[derive(Component)]
pub struct PowerupBarContainer;

#[derive(Component)]
pub struct PowerupBar;

pub struct PaintFont(Handle<Font>);

pub fn ui_setup(mut commands: Commands, font: Res<PaintFont>) {
    let font = &font.0;

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(20.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("UI"))
        .insert(PlayerUi)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_grow: 1.0,
                        flex_shrink: 1.0,
                        flex_basis: Val::Percent(100.0),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(Name::new("LeftSection"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(240.0), Val::Auto),
                                margin: UiRect::new(
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                    Val::Px(10.0),
                                ),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .insert(Name::new("Bars"))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
                                        padding: UiRect::all(Val::Px(7.0)),
                                        ..Default::default()
                                    },
                                    color: Color::BLACK.into(),
                                    ..Default::default()
                                })
                                .insert(Name::new("HealthBarContainer"))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.0),
                                                    Val::Percent(100.0),
                                                ),
                                                ..Default::default()
                                            },
                                            color: Color::rgb(0.95, 0.04, 0.07).into(),
                                            ..Default::default()
                                        })
                                        .insert(Name::new("HealthBar"))
                                        .insert(HealthBar);
                                });
                        });

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(240.0), Val::Percent(20.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .insert(Name::new("Inventory"))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "Cocaine: ",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 32.0,
                                            color: Color::PINK, // TODO: Give it a sensible color (and maybe change the font)
                                        },
                                    )
                                    .with_style(Style {
                                        size: Size::new(Val::Auto, Val::Auto),
                                        margin: UiRect::all(Val::Px(0.0)),
                                        ..default()
                                    }),
                                )
                                .insert(Name::new("SmallPowerUpCounter"))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(
                                            TextBundle::from_section(
                                                "0",
                                                TextStyle {
                                                    font: font.clone(),
                                                    font_size: 32.0,
                                                    color: Color::PINK, // this needs changing
                                                },
                                            )
                                            .with_style(Style {
                                                size: Size::new(Val::Auto, Val::Auto),
                                                margin: UiRect::new(
                                                    Val::Px(115.0),
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                ),
                                                ..default()
                                            }),
                                        )
                                        .insert(Name::new("SmallPowerUpCounterNumber"))
                                        .insert(SmallPowerUpCounterNumber);
                                });

                            parent
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "Cocaine+1: ",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 32.0,
                                            color: Color::PINK,
                                        },
                                    )
                                    .with_style(Style {
                                        size: Size::new(Val::Auto, Val::Auto),
                                        margin: UiRect::all(Val::Px(0.0)),
                                        ..default()
                                    }),
                                )
                                .insert(Name::new("BigPowerUpCounter"))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(
                                            TextBundle::from_section(
                                                "0",
                                                TextStyle {
                                                    font: font.clone(),
                                                    font_size: 32.0,
                                                    color: Color::PINK, // this needs changing
                                                },
                                            )
                                            .with_style(Style {
                                                size: Size::new(Val::Auto, Val::Auto),
                                                margin: UiRect::new(
                                                    Val::Px(155.0),
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                ),
                                                ..default()
                                            }),
                                        )
                                        .insert(Name::new("BigPowerUpCounterNumber"))
                                        .insert(BigPowerUpCounterNumber);
                                });
                        });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_grow: 1.0,
                        flex_shrink: 1.0,
                        flex_basis: Val::Percent(100.0),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(Name::new("MiddleSection"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: UiRect::new(
                                    Val::Px(7.0),
                                    Val::Px(7.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                ),
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .insert(Name::new("Bars"))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
                                        padding: UiRect::all(Val::Px(7.0)),
                                        ..Default::default()
                                    },
                                    color: Color::BLACK.into(),
                                    ..Default::default()
                                })
                                .insert(Name::new("PowerupBarContainer"))
                                .insert(PowerupBarContainer)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.0),
                                                    Val::Percent(100.0),
                                                ),
                                                ..Default::default()
                                            },
                                            color: Color::WHITE.into(),
                                            ..Default::default()
                                        })
                                        .insert(Name::new("PowerupBar"))
                                        .insert(PowerupBar);
                                });
                        });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        flex_shrink: 1.0,
                        flex_basis: Val::Percent(100.0),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(Name::new("RightSection"));
        });
}

pub fn drop_ui(mut commands: Commands, ui_query: Query<Entity, With<PlayerUi>>) {
    let ui = ui_query.single();
    commands.entity(ui).despawn_recursive();
}

pub fn update_ui(
    player_query: Query<(&Health, &Inventory, &EffectData), With<Player>>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
    mut small_powerup_counter_query: Query<
        &mut Text,
        (
            With<SmallPowerUpCounterNumber>,
            Without<BigPowerUpCounterNumber>,
        ),
    >,
    mut big_powerup_counter_query: Query<
        &mut Text,
        (
            With<BigPowerUpCounterNumber>,
            Without<SmallPowerUpCounterNumber>,
        ),
    >,
    mut powerup_bar_container_query: Query<
        &mut Style,
        (With<PowerupBarContainer>, Without<HealthBar>),
    >,
    mut powerup_bar_query: Query<
        &mut Style,
        (
            With<PowerupBar>,
            Without<PowerupBarContainer>,
            Without<HealthBar>,
        ),
    >,
) {
    let (player_health, inventory, effect_data) = player_query.single();

    let mut health_bar_style = health_bar_query.single_mut();
    health_bar_style.size.width =
        Val::Percent(player_health.get_health() / player_health.get_max_health() * 100.0);

    let mut small_powerup_counter = small_powerup_counter_query.single_mut();
    small_powerup_counter.sections[0].value = inventory.get_small_powerup_quantity().to_string();

    let mut big_powerup_counter = big_powerup_counter_query.single_mut();
    big_powerup_counter.sections[0].value = inventory.get_big_powerup_quantity().to_string();

    let mut powerup_bar_container = powerup_bar_container_query.single_mut();
    let mut powerup_bar = powerup_bar_query.single_mut();

    if effect_data.effect.is_some() {
        powerup_bar_container.display = Display::Flex;
        powerup_bar.size.width = Val::Percent(
            100.0
                * (1.0
                    - effect_data.duration.elapsed_secs()
                        / effect_data.duration.duration().as_secs_f32()),
        );
    } else {
        powerup_bar_container.display = Display::None;
    }
}

pub fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PaintFont(asset_server.load("floyd.ttf")));
}
