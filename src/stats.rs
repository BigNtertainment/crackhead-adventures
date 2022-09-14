use bevy::{prelude::*, time::Stopwatch};

use crate::GameState;

pub struct StatsPlugin;

pub struct Stats{
    pub timer: Stopwatch,
    pub enemies_killed: u16,
    pub small_powerup_used: u16,
    pub small_powerup_collected: u16,
    pub big_powerup_used: u16,
    pub big_powerup_crafted: u16,
    pub damage_taken: f32,
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
            damage_taken:0.0,
        })
           .add_system_set(SystemSet::on_enter(GameState::Game).with_system(start_timer))
           .add_system_set(SystemSet::on_exit(GameState::Game).with_system(pause_timer))
           .add_system_set(SystemSet::on_update(GameState::Game).with_system(update_timer));
        
        
    }
}


fn start_timer(mut stats: ResMut<Stats>) {
    stats.timer.reset();
    stats.timer.unpause();
}

fn pause_timer(mut stats: ResMut<Stats>) {
    stats.timer.pause();
}

fn update_timer(mut stats: ResMut<Stats>, time: Res<Time>) {
    stats.timer.tick(time.delta());
}
