use bevy::prelude::*;
use std::fmt::{Display, Formatter};
use crate::game::ball::{Ball, create_ball};
use crate::config::BALL_SIZE;
use std::{thread, time};

#[derive(Default)]
pub struct Score {
	pub left: usize,
	pub right: usize,
}

impl Display for Score {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
	write!(formatter, "{} : {}", self.left, self.right)
    }
}

pub struct ScoreBoard;

pub struct ScoreEvent {
    message: Scorer,
}

pub enum Scorer {
    Left,
    Right,
}

pub fn spawn_score_board(commands: &mut Commands,
			 asset_server: Res<AssetServer>,
			 score: ResMut<Score>) {
    commands
	.spawn()
	.insert_bundle(TextBundle {
	    style: Style {
		size: Size::new(Val::Px(100.0), Val::Px(50.0)),
		// center
		margin: Rect {
		    top: Val::Px(550.0),
		    ..Rect::all(Val::Auto)
		},
		..Default::default()
	    },
	    text: Text {
		sections: vec![TextSection {
		    value: score.to_string(),
		    style: TextStyle {
			font_size: 60.0,
			font: asset_server.load("fonts/FiraSans-Bold.ttf"),
			..Default::default()
		    },
		}],
		alignment: Default::default(),
	    },
	    ..Default::default()
	})
	.insert(ScoreBoard);
}

pub fn scoring(windows: Res<Windows>,
	       mut score_event: EventWriter<ScoreEvent>,
	       mut commands: Commands,
	       mut query: Query<(Entity, &mut Ball, &Transform)>) {
    let window = windows.get_primary().unwrap();
    if let Ok((entity, mut ball, transform)) = query.single_mut() {
	let bound = (window.height() / 2.0) - (BALL_SIZE / 2.0);
	if transform.translation.abs().y > bound {
	    ball.velocity.y = -1.0 * ball.velocity.y;
	}
	if transform.translation.x > window.width() / 2.0 {
	    commands.entity(entity).despawn_recursive();
	    score_event.send(ScoreEvent { message: Scorer::Left });
	} else if transform.translation.x < -(window.width() / 2.0) {
	    commands.entity(entity).despawn_recursive();
	    score_event.send(ScoreEvent { message: Scorer::Right});
	}
    }
}

pub fn score_listener(mut events: EventReader<ScoreEvent>,
		      mut score: ResMut<Score>,
		      mut commands: Commands,
		      mut query: Query<&mut Text>) {
    for event in events.iter() {
	if let Ok(mut text) = query.single_mut() {
	    match event.message {
		Scorer::Left => score.left += 1,
		Scorer::Right => score.right += 1
	    }
	    text.sections[0].value = score.to_string();
	    thread::sleep(time::Duration::from_millis(100));
	    commands.spawn_bundle(create_ball())
		.insert(Ball {
		    velocity: Vec2::new(100.0, 50.0)
		});
	}
    }
}
