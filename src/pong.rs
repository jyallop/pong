use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::game::ball::*;
use crate::game::paddle::*;
use crate::game::scoring::scoring;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut AppBuilder) {
	app.add_startup_system(setup.system())
	    .add_startup_system(player_setup.system())
	    .add_startup_system(computer_setup.system())
	    .add_startup_system(ball_setup.system())
	    .add_system(paddle_movement.system())
	    .add_system(computer_movement.system())
	    .add_system(ball_movement.system())
	    .add_system(scoring.system())
	    .add_system(collision_detection.system());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn collision_detection(paddles: Query<(&Transform, &Sprite), With<Paddle>>,
		       mut ball: Query<(&mut Ball, &Transform, &Sprite)>) {
    if let Ok((mut ball, transform, ball_sprite)) = ball.single_mut() {
	for (paddle_transform, sprite) in paddles.iter() {
	    if let Some(collision) = collide(transform.translation,
					     ball_sprite.size,
					     paddle_transform.translation,
					     sprite.size) {
		match collision {
		    Collision::Left =>  ball.velocity.x = -1.5 * ball.velocity.x,
		    Collision::Right => ball.velocity.x = -1.5 * ball.velocity.x,
		    _ => ball.velocity.y = -1.0 * ball.velocity.y,
		}
	    }	
	}
    }
}

