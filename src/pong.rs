use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::game::ball::*;
use crate::game::paddle::*;
use crate::game::scoring::*;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut AppBuilder) {
	app.add_startup_system(setup.system())
	    .add_event::<ScoreEvent>()
	    .insert_resource(Score { left: 0, right: 0 })
	    .add_startup_system(player_setup.system())
	    .add_startup_system(computer_setup.system())
	    .add_startup_system(ball_setup.system())
	    .add_system(score_listener.system())
	    .add_system(paddle_movement.system())
	    .add_system(computer_movement.system())
	    .add_system(ball_movement.system())
	    .add_system(scoring.system())
	    .add_system(collision_detection.system());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, score: ResMut<Score>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    spawn_score_board(&mut commands, asset_server, score);
}

fn collision_detection(paddles: Query<(&Transform, &Sprite), With<Paddle>>,
		       windows: Res<Windows>,
		       mut ball: Query<(&mut Ball, &Transform, &Sprite)>) {
    if let (Ok((mut ball, transform, ball_sprite)), window) = (ball.single_mut(), windows.get_primary().unwrap()) {
	for (paddle_transform, sprite) in paddles.iter() {
	    if let Some(collision) = collide(transform.translation,
					     ball_sprite.size,
					     paddle_transform.translation,
					     sprite.size) {
		match collision {
		    Collision::Left =>  ball.velocity.x = (-1.1 * ball.velocity.x).min(window.width() * 0.45),
		    Collision::Right => ball.velocity.x = (-1.1 * ball.velocity.x).min(window.width() * 0.45),
		    _ => ball.velocity.y = -1.0 * ball.velocity.y,
		}
	    }	
	}
    }
}

