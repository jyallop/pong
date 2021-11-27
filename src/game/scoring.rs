use bevy::prelude::*;
use crate::game::ball::Ball;
use crate::config::BALL_SIZE;

pub fn scoring(windows: Res<Windows>,
	       mut query: Query<(&mut Ball, &Transform)>) {
    let window = windows.get_primary().unwrap();
    if let Ok((mut ball, transform)) = query.single_mut() {
	let bound = (window.height() / 2.0) - (BALL_SIZE / 2.0);
	if transform.translation.abs().y > bound {
	    ball.velocity.y = -1.0 * ball.velocity.y;
	}
    }
}

