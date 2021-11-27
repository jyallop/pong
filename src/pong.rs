use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

pub struct PongPlugin;

static BALL_SIZE: f32 = 30.0;
static PADDLE_WIDTH: f32 = 30.0;
static PADDLE_HEIGHT: f32 = 120.0;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut AppBuilder) {
	app.add_startup_system(setup.system())
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

fn paddle_movement(time: Res<Time>,
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

fn scoring(windows: Res<Windows>,
	   mut query: Query<(&mut Ball, &Transform)>) {
    let window = windows.get_primary().unwrap();
    if let Ok((mut ball, transform)) = query.single_mut() {
	let bound = (window.height() / 2.0) - (BALL_SIZE / 2.0);
	if transform.translation.abs().y > bound {
	    ball.velocity.y = -1.0 * ball.velocity.y;
	}
    }
}

fn ball_movement(time: Res<Time>,
		 mut query: Query<(&mut Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = query.single_mut() {
	let translation = &mut transform.translation;
	translation.x += time.delta_seconds() * ball.velocity.x;
	translation.y += time.delta_seconds() * ball.velocity.y;
    }
}

fn computer_movement(time: Res<Time>,
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

