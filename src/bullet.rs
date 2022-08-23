use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, Sensor};

use crate::GameState;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_event::<ShotEvent>()

			.add_startup_system(load_bullet_texture)
			
			.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(update_bullets.label("update_bullets"))
                .with_system(drop_bullets_on_collision.after("update_bullets")),
        );
    }
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub bullet: Bullet,
    pub collider: Collider,
    pub sensor: Sensor,
    pub name: Name,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            bullet: Bullet { speed: 4000.0 },
            collider: Collider::cuboid(10.0, 10.0),
            sensor: Sensor,
            sprite_bundle: SpriteBundle::default(),
            name: Name::new("Bullet"),
        }
    }
}

pub struct ShotEvent(pub Entity);

#[derive(Deref, DerefMut)]
pub struct BulletTexture(Handle<Image>);

fn load_bullet_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BulletTexture(asset_server.load("img/bullet.png")));
}

fn update_bullets(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        let direction = transform.up();

        let movement_vector = direction * bullet.speed * time.delta_seconds();

        transform.translation += movement_vector;
    }
}

pub fn drop_bullets_on_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &Collider), With<Bullet>>,
    rapier_context: Res<RapierContext>,
    mut event_shot: EventWriter<ShotEvent>,
) {
    for (bullet, bullet_transform, bullet_collider) in bullets.iter() {
        let shape = bullet_collider;
        let position = bullet_transform.translation.truncate();
        let rotation = bullet_transform.rotation.z;
        let filter = QueryFilter::default().exclude_sensors();

        rapier_context.intersections_with_shape(position, rotation, shape, filter, |collider| {
			commands.entity(bullet).despawn_recursive();

			event_shot.send(ShotEvent(collider));

			true
		});
    }
}
