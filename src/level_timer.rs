use bevy::{prelude::*, time::Stopwatch};

use crate::GameState;



pub struct LevelTimerPlugin;

#[derive(Deref,DerefMut)]
pub struct LevelTimer(Stopwatch);


impl Plugin for LevelTimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelTimer(Stopwatch::new()))
           .add_system_set(SystemSet::on_enter(GameState::Game).with_system(start_timer))
           .add_system_set(SystemSet::on_exit(GameState::Game).with_system(pause_timer))
           .add_system_set(SystemSet::on_update(GameState::Game).with_system(update_timer));
        
        
    }
}


fn start_timer(mut level_timer: ResMut<LevelTimer>) {
    level_timer.reset();
    level_timer.unpause();
}

fn pause_timer(mut level_timer: ResMut<LevelTimer>) {
    level_timer.pause();
}

fn update_timer(mut level_timer: ResMut<LevelTimer>, time: Res<Time>) {
    level_timer.tick(time.delta());
    println!("{}", level_timer.elapsed_secs());
}
