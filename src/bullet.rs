use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, Sensor};

use crate::GameState;

pub const BULLET_COLLIDER_WIDTH: f32 = 10.0;
pub const BULLET_COLLIDER_HEIGHT: f32 = 10.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<ShotEvent>()
			.add_startup_system(load_bullet_texture)
			.add_system_set(
				SystemSet::on_update(GameState::Game)
					.with_system(update_bullets.label("update_bullets"))
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
			collider: Collider::cuboid(BULLET_COLLIDER_WIDTH, BULLET_COLLIDER_HEIGHT),
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
	commands.insert_resource(BulletTexture(asset_server.load("./img/bullet.png")));
}

fn update_bullets(
	mut commands: Commands,
	mut bullets: Query<(Entity, &mut Transform, &Bullet)>,
	time: Res<Time>,
	rapier_context: Res<RapierContext>,
	mut event_shot: EventWriter<ShotEvent>,
) {
	for (bullet_entity, mut transform, bullet) in bullets.iter_mut() {
		let direction = transform.up();

		let movement_vector = direction * bullet.speed * time.delta_seconds();

		let filter = QueryFilter::default().exclude_sensors();

		// Check for collisions
		if let Some((hit_entity, _)) = rapier_context.cast_ray(
			transform.translation.truncate(),
			movement_vector.truncate(),
			1.0,
			true,
			filter,
		) {
			commands.entity(bullet_entity).despawn_recursive();

			event_shot.send(ShotEvent(hit_entity));

			continue;
		}

		// If no collision occured, move
		transform.translation += movement_vector;
	}
}
