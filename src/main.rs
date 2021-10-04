mod game;

use iced::{
    canvas::{self, Cursor, Path, Stroke},
    executor, time, window, Application, Canvas, Clipboard, Color, Command,
    Element, Length, Point, Rectangle, Settings, Size, Subscription, Vector,
};

use std::time::Instant;
use game::Game;

pub fn main() -> iced::Result {
    Game::run(Settings {
	window: iced::window::Settings {
	    size: (400, 400),
	    min_size: None,
	    max_size: None,
	    resizable: false,
	    decorations: true,
	    transparent: false,
	    always_on_top: false,
	    icon: None,
	},
        flags: (),
        default_font: None,
        default_text_size: 16,
        exit_on_close_request: true,
        antialiasing: false,
	
    })
}

