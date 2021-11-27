use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

pub struct PongPlugin;

static BALL_SIZE: f32 = 30.0;
static PADDLE_WIDTH: f32 = 30.0;
static PADDLE_HEIGHT: f32 = 120.0;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut AppBuilder) {
	app.add_startup_system(setup.system())
	    .add_system(paddle_movement_system.system())
	    .add_system(computer_movement_system.system())
	    .add_system(ball_movement_system.system())
	    .add_system(collision_detection.system());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-600.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        })
	.insert(Paddle)
	.insert(Player);

    // computer paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(600.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        })
	.insert(Paddle)
	.insert(Computer);

    // ball
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..Default::default()
        })
	.insert(Ball {
	    velocity: Vec2::new(-100.0, -50.0)
	});
}

pub struct Paddle;
pub struct Player;
pub struct Computer;
pub struct Ball {
    velocity: Vec2,
}

fn paddle_movement_system(time: Res<Time>,
			  keyboard_input: Res<Input<KeyCode>>,
			  mut query: Query<(&Player, &mut Transform)>) {
    if let Ok((_paddle, mut transform)) = query.single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            direction += 100.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= 100.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.y += time.delta_seconds() * direction * 5.0;
        // bound the paddle within the walls
        translation.y = translation.y.min(300.0).max(-300.0);
    }
}

fn ball_movement_system(time: Res<Time>,
			mut query: Query<(&Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = query.single_mut() {
	let translation = &mut transform.translation;
	translation.x += time.delta_seconds() * ball.velocity.x;
	translation.y += time.delta_seconds() * ball.velocity.y;
    }
}

fn computer_movement_system(time: Res<Time>,
			    mut queries: QuerySet<(
				Query<(&Computer, &mut Transform)>,
				Query<(&Ball, &Transform)>)>) {
    let mut ball_center = Vec2::new(0.0, 0.0);
    if let Ok((_ball, transform)) = queries.q1().single() {
	ball_center.x = transform.translation.x + (BALL_SIZE / 2.0);
	ball_center.y = transform.translation.y + (BALL_SIZE / 2.0);
    }

    if let Ok((_computer, mut transform)) = queries.q0_mut().single_mut() {
	let movement;
	let paddle_location = &mut transform.translation;
	if paddle_location.y + (PADDLE_HEIGHT / 2.0) > ball_center.y {
	    movement = -100.0;
	} else {
	    movement = 100.0;
	}
	paddle_location.y += time.delta_seconds() * movement;
	paddle_location.y = paddle_location.y.min(300.0).max(-300.0);
    }
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

