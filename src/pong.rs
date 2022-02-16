use bevy::prelude::*;
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
	app.add_startup_system(setup.system())
	    .add_event::<ScoreEvent>()
	    .add_state(GameState::Paused)
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

pub fn pause_listener(windows: Res<Windows>,
		      mut state: ResMut<State<GameState>>,
		      key: Res<Input<KeyCode>>) {
    if key.pressed(KeyCode::Space) {
	if *state.current() == GameState::Paused {
	    let _result = state.pop();
	} else {
	    let _result = state.push(GameState::Paused);
	}
    }

}

   

