use bevy::{
    app::{App, Startup, Update},
    asset::{AssetMode, AssetPlugin, AssetServer},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, Resource},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec2,
    prelude::*,
    time::Time,
    transform::components::Transform,
};

use crate::components::*;

pub fn input_read(
    keymap: Res<KeyMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    let mut direction = Vec2::default();

    if keyboard_input.pressed(keymap.move_up) {
        direction += Vec2::new(0.0, 1.0);
    }

    if keyboard_input.pressed(keymap.move_down) {
        direction += Vec2::new(0.0, -1.0);
    }

    if keyboard_input.pressed(keymap.move_left) {
        direction += Vec2::new(-1.0, 0.0);
    }

    if keyboard_input.pressed(keymap.move_right) {
        direction += Vec2::new(1.0, 0.0);
    }

    let mut player_velocity = player_query.single_mut();
    player_velocity.0 = direction.normalize_or_zero();
}
