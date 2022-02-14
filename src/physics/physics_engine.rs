use crate::game::ball::*;
use crate::game::paddle::*;
use bevy_math::{Vec2, Vec3};
use bevy::prelude::*;

#[derive(Debug)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

pub fn collision_detection(paddles: Query<(&Transform, &Sprite), With<Paddle>>,
		       windows: Res<Windows>,
		       mut ball: Query<(&mut Ball, &Transform, &Sprite)>) {
    if let (Ok((mut ball, transform, ball_sprite)), window) = (ball.single_mut(), windows.get_primary().unwrap()) {
	for (paddle_transform, sprite) in paddles.iter() {
	    if let Some((collision, hit_spot)) = collide(paddle_transform.translation,
					     sprite.size,
					     transform.translation,
					     ball_sprite.size) {
		match collision {
		    Collision::Left =>  ball.velocity.x = (-1.1 * ball.velocity.x).min(window.width() * 0.45),
		    Collision::Right => ball.velocity.x = (-1.1 * ball.velocity.x).min(window.width() * 0.45),
		    _ => ball.velocity.y = -1.0 * ball.velocity.y,
		}
		ball.adjust_angle(hit_spot);
	    }	
	}
    }
}

fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<(Collision, f32)> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
	let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x {
	    (Collision::Left, b_min.x - a_max.x)
	} else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
	    (Collision::Right, a_min.x - b_max.x)
	} else {
	    (Collision::Inside, -f32::INFINITY)
	};

	let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y {
	    (Collision::Bottom, b_min.y - a_max.y)
	} else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
	    (Collision::Top, a_min.y - b_max.y)
	} else {
	    (Collision::Inside, -f32::INFINITY)
	};

	let hit_spot = (b_pos.y - a_pos.y) / (a_size.y / 2.0);
	if y_depth.abs() < x_depth.abs() {
	    Some((y_collision, hit_spot))
	} else {
	    Some((x_collision, hit_spot))
	}
    } else {
	None
    }
}
