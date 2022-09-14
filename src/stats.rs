use bevy::{prelude::*, time::Stopwatch};

use crate::GameState;

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
           .add_system_set(SystemSet::on_update(GameState::Game).with_system(update_stats));
        
        
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