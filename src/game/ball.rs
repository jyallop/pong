use bevy::prelude::*;
use rand::{Rng};
use crate::config::{BALL_SIZE, BALL_SPEED};
use crate::game::scoring::Scorer;

pub fn ball_setup(mut commands: Commands) {
     commands
        .spawn_bundle(create_ball_sprite())
	.insert(create_ball(&Scorer::Left));
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

pub fn create_ball_sprite() -> SpriteBundle {
    SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
	sprite: Sprite::new(Vec2::new(BALL_SIZE, BALL_SIZE)),
        ..Default::default()
    }
}

pub fn create_ball(scorer: &Scorer) -> Ball {
    let angle = rand::thread_rng().gen_range(-100.0..100.0);
    let speed = match scorer {
	&Scorer::Left => -BALL_SPEED,
	&Scorer::Right => BALL_SPEED
    };
    Ball {
	velocity: Vec2::new(speed, angle)
    }
}

impl Ball {
    pub fn adjust_angle(&mut self, angle: f32) -> () {
	self.velocity.y = if self.velocity.y > 0.0 {
	    self.velocity.y * (1.0 + angle)
	} else {
	    self.velocity.y * (1.0 - angle)
	}
    }
}
