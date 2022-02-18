use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy::input::{keyboard::KeyboardInput, ElementState};
use crate::game::ball::*;
use crate::game::paddle::*;
use crate::game::scoring::*;
use crate::physics::physics_engine::*;

pub struct PongPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Running,
    Paused,
}

impl Plugin for PongPlugin {
    fn build(&self, app: &mut AppBuilder) {
	app.add_state(GameState::Running)
	    .add_plugin(Game)
	    .add_plugin(Pause)
	    .add_system(pause_listener.system());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub struct Game;
pub struct Pause;

impl Plugin for Game {
    fn build(&self, app: &mut AppBuilder) {
	app.add_system_set(SystemSet::on_enter(GameState::Running).with_system(setup.system()))
	    .add_system_set(SystemSet::on_pause(GameState::Running))
	    .add_system_set(SystemSet::on_resume(GameState::Running))
	    .insert_resource(Score { left: 0, right: 0 })
	    .add_event::<ScoreEvent>()
	    .add_startup_system(score_board.system())
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

pub struct PausedScreen;

impl Plugin for Pause {
    fn build(&self, app: &mut AppBuilder) {
	println!("Entered Pause");
	app.add_system_set(SystemSet::on_enter(GameState::Paused).with_system(pause_setup.system()))
	    .add_system_set(SystemSet::on_exit(GameState::Paused)
			    .with_system(despawn_screen::<PausedScreen>.system()));
    }
}

fn pause_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_alignment = TextAlignment {
	vertical: VerticalAlign::Center,
	horizontal: HorizontalAlign::Center,
    };
    let text_style = TextStyle {
	font,
	font_size: 60.0,
	color: Color::WHITE,
    };
    commands.spawn_bundle(Text2dBundle {
	text: Text::with_section("Paused", text_style, text_alignment),
	..Default::default()
    })
	.insert(PausedScreen);
}

pub fn pause_listener(mut state: ResMut<State<GameState>>,
		      mut input: EventReader<KeyboardInput>) {
    for event in input.iter() {
	event.key_code.map(|keycode| 
			   if event.state == ElementState::Released && keycode == KeyCode::Space {
			       if *state.current() == GameState::Paused {
				   let _result = state.pop();
			       } else {
				   let _result = state.push(GameState::Paused);
			       }
			   });
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
	commands.entity(entity).despawn_recursive();
    }
}
