use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build (&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}

fn startup() {

}
