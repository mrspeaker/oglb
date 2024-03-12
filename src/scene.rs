use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use std::f32::consts::PI;
use crate::rocket::RocketOne;
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>
) {
    // Sun
    cmds.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            //color: Color::rgb(1.0, 1.0, 1.0),
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(1.0, 1.0, 0.0),
            rotation: Quat::from_rotation_x(-PI * 0.5),
            ..default()
        },
        ..default()
    });
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });

    //ocean
    cmds.spawn(SceneBundle {
        scene: assets.load("ocean/scene.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, -180.0, -900.0)
            .with_scale(Vec3::ONE * 300.0),
        ..default()
    });

    // mountain
        cmds.spawn(SceneBundle {
        scene: assets.load("mountain/scene.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, -3000.0)
            .with_scale(Vec3::ONE * 1000.0),
        ..default()
    });

    // Droneships
    for i in 0..2 {
        cmds.spawn((
            PbrBundle {
                transform: Transform::from_xyz(i as f32 * 160.0 + -80.0, 0.0, 0.0),
                mesh: meshes.add(Mesh::from(Cuboid::new(80.0, 8.0, 80.0))),
                material: mats.add(Color::rgb(1., 0.9, 0.8)),
                ..default()
            },
            RigidBody::Static,
            Collider::cuboid(80.0, 8.0, 80.0),
        ));
    }

    cmds.spawn(SceneBundle {
        scene: assets.load("booster/scene.gltf#Scene0"),
        transform: Transform::from_xyz(80.0, 35.0 + 8.0, 0.0)
            .with_scale(Vec3::ONE * 5.0),
        ..default()
    });

    // Rocket one
    cmds.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-80.0, 35.0 + 8.0, 0.0),
            mesh: meshes.add(Mesh::from(Cylinder::new(3.7, 70.0))),
            material: mats.add(Color::rgb(1., 0.9, 0.8)),

            ..default()
        },
        RigidBody::Dynamic,
        //AngularVelocity(Vec3::new(0.0, 0.0, 0.5)),
        //LinearVelocity(Vec3::new(0.0, 40.0, 0.0)),
        Collider::cylinder(70.0, 3.7),
//        Mass(5000.0),
        RocketOne
    ));

    cmds.spawn((
        PbrBundle {
            transform: Transform::from_xyz(80.0, 35.0 + 8.0, 0.0),
            mesh: meshes.add(Mesh::from(Cylinder::new(3.7, 70.0))),
            material: mats.add(Color::rgb(1., 0.9, 0.8)),
            ..default()
        },
        RigidBody::Dynamic,
        //AngularVelocity(Vec3::new(0.0, 0.0, 0.5)),
        //LinearVelocity(Vec3::new(0.0, 40.0, 0.0)),
        Collider::cylinder(70.0, 3.7),
    ));
}
