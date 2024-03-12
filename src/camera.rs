use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}

fn startup(
    mut cmds: Commands
) {
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 120.0, 300.0)
                .looking_at(Vec3::ZERO + (Vec3::Y * 50.0), Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.15, 0.15, 0.15, 1.0),
            falloff: FogFalloff::Linear {
                start: 1500.0,
                end: 2900.0,
            },
            ..default()
        },
        MainCamera
    ));
}
