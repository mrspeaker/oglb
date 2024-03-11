mod game;
mod camera;
mod scene;
mod player;
mod states;
mod rocket;

use game::GamePlugin;
use camera::CameraPlugin;
use scene::ScenePlugin;
use player::PlayerPlugin;

use states::GameState;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((
            DefaultPlugins,
            GamePlugin,
            CameraPlugin,
            ScenePlugin,
            PlayerPlugin,
            PhysicsPlugins::default()
        ))
        .run();
}
