use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::Q) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::E) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);

    transform.rotate_local_z(roll);

    velocity.value = -transform.forward() * movement;
}
