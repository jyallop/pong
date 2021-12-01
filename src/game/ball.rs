use bevy::prelude::*;
use crate::config::BALL_SIZE;

pub fn ball_setup(mut commands: Commands) {
    commands
        .spawn_bundle(create_ball())
	.insert(Ball {
	    velocity: Vec2::new(-200.0, -50.0)
	});
}

pub struct Ball {
    pub velocity: Vec2,
}

pub fn ball_movement(time: Res<Time>,
		     mut query: Query<(&mut Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = query.single_mut() {
	let translation = &mut transform.translation;
	translation.x += time.delta_seconds() * ball.velocity.x;
	translation.y += time.delta_seconds() * ball.velocity.y;
    }
}

pub fn create_ball() -> SpriteBundle {
    SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
	sprite: Sprite::new(Vec2::new(BALL_SIZE, BALL_SIZE)),
        ..Default::default()
    }
}
