use bevy::prelude::*;
use crate::config::{PADDLE_WIDTH, PADDLE_HEIGHT};
use crate::game::ball::Ball;

pub struct Paddle;
pub struct Player;
pub struct Computer;

pub fn player_setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-600.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        })
	.insert(Paddle)
	.insert(Player);
}

pub fn computer_setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(600.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        })
	.insert(Paddle)
	.insert(Computer);
}

pub fn paddle_movement(time: Res<Time>,
		   windows: Res<Windows>,
		   keyboard_input: Res<Input<KeyCode>>,
		   mut query: Query<(&Player, &mut Transform)>) {
    if let Ok((_paddle, mut transform)) = query.single_mut() {
	let window = windows.get_primary().unwrap();
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            direction = window.height() / 10.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction = -(window.height() / 10.0);
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.y += time.delta_seconds() * direction * 5.0;
        // bound the paddle within the walls
	let max = (window.height() / 2.0) - (PADDLE_HEIGHT / 2.0);
        translation.y = translation.y.min(max).max(-max);
    }
}

pub fn computer_movement(time: Res<Time>,
		     windows: Res<Windows>,
		     mut queries: QuerySet<(Query<(&Computer, &mut Transform)>,
					    Query<(&Ball, &Transform)>)>) {
    let mut ball_center = Vec2::new(0.0, 0.0);
    let window = windows.get_primary().unwrap();
    if let Ok((_ball, transform)) = queries.q1().single() {
	ball_center.x = transform.translation.x;
	ball_center.y = transform.translation.y;
    }

    if let Ok((_computer, mut transform)) = queries.q0_mut().single_mut() {
	let movement;
	let paddle_location = &mut transform.translation;
	if paddle_location.y > ball_center.y {
	    movement = -(window.height() / 10.0);
	} else {
	    movement = window.height() / 10.0;
	}
	paddle_location.y += time.delta_seconds() * movement;
	let max = (window.height() / 2.0) - (PADDLE_HEIGHT / 2.0);
	paddle_location.y = paddle_location.y.min(max).max(-max);
    }
}

