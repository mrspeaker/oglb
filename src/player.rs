use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_xpbd_3d::prelude::*;
use crate::rocket::RocketOne;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q: Query<(&Transform, &mut LinearVelocity, &mut AngularVelocity), With<RocketOne>>)
{
    let Ok((t, mut vel, mut ang)) = q.get_single_mut() else {
        return;
    };
    let dt = time.delta_seconds();

    let mut rot = Vec3::ZERO;

    for event in mouse_events.read() {
        rot.x = -event.delta.y; // Pitch
        rot.y = -event.delta.x; // Yaw
    }

    let mut imp = Vec3::ZERO;
    let mut rot = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) { imp += Vec3::from(t.up()) }
    if keys.pressed(KeyCode::KeyQ) { rot -= Vec3::Z }
    if keys.pressed(KeyCode::KeyE) { rot += Vec3::Z }

    if imp.length() > 0.0 {
        //t.translation += imp * dt * 10.0;
        const SPEED: f32 = 20.0;
        //impulse.add_force(imp.normalize() * SPEED * dt);
        vel.x += imp.x * SPEED * dt;
        vel.y += imp.y * SPEED * dt;
        vel.z += imp.z * SPEED * dt;
    }

    if rot.length() > 0.0 {
        const SPEED: f32 = 1.;
        //torque.add_force(rot * SPEED * dt);
        ang.x += rot.x * SPEED * dt;
        ang.y += rot.y * SPEED * dt;
        ang.z += rot.z * SPEED * dt;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        //
    }
}
