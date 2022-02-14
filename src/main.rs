mod pong;
mod game;
mod config;
mod physics;

use bevy::prelude::*;
use crate::pong::PongPlugin;

pub fn main() {
    App::build()
	.add_plugins(DefaultPlugins)
	.add_plugin(PongPlugin)
	.run();
}


