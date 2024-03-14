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
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "One Giant Loop".into(),
                    resolution: (800., 350.).into(),
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
//            EditorPlugin::default(),
            CameraPlugin,
            ScenePlugin,
            PlayerPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .run();
}
