mod paddle;
mod ball;
mod drawable;

use iced::{time, Application, Command, Container, Element, Length, Point, Size, canvas::{Cursor, Event, event::{self, Status}}, executor, keyboard::KeyCode};
use iced::canvas::{self, Canvas, Frame, Geometry, Path, Program};
use iced::{Color, Rectangle};
use paddle::Paddle;
use ball::Ball;
use std::time::{Duration, Instant};
use drawable::Drawable;
use std::boxed::Box;

use self::paddle::Direction;

pub struct Game {
    game_objects: Vec<Box<dyn Drawable>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    None,
    MoveUp,
    MoveDown,
    Tick(Instant),
}

impl Application for Game {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
	(Game { game_objects: vec![ Box::new(Paddle::new(0.02, 0.1, 0.02, 0.10)),
				    Box::new(Paddle::new(0.96, 0.1, 0.02, 0.10)),
				    Box::new(Ball::new(0.5, 0.5, 0.01)) ] },
	 Command::none())
    }

    fn title(&self) -> String {
	String::from("Pong")
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
	match message {
	    _ => Command::none(),
//	    Message::MoveDown => { self.paddle_one.slide(Direction::Down); Command::none() },
	    //	    Message::MoveUp => { self.paddle_one.slide(Direction::Up); Command::none() },
	    //	    Message::Tick(_) => { self.ball.move_ball(); Command::none() },
	}
    }

    fn view(&mut self) -> Element<Message> {
	let canvas = Canvas::new(self)
	    .width(Length::Units(400))
	    .height(Length::Units(400));

	Container::new(canvas)
	    .width(Length::Fill)
	    .height(Length::Fill)
	    .into()

    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
	time::every(Duration::from_millis(100)).map(Message::Tick)
    }
}

impl Program<Message> for Game {
    fn draw(&self, bounds: iced::Rectangle, cursor: iced::canvas::Cursor) -> Vec<iced::canvas::Geometry> {
                // We prepare a new `Frname`
        let mut frame = Frame::new(bounds.size());

	let objects = &self.game_objects;
	
	objects.into_iter().map(|drawable_pointer| drawable_pointer.draw(bounds))
	    .for_each(|path| frame.fill(&path, Color::BLACK));

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }

    fn update(&mut self, event: Event, _bounds: Rectangle<f32>, _cursor: Cursor) -> (Status, Option<Message>) {
	println!("Event: {:?}", event);
	let message = match event {
	    Event::Keyboard(event) => match event {
		iced::keyboard::Event::KeyPressed { key_code: KeyCode::Down, ..} => Message::MoveDown,
		iced::keyboard::Event::KeyPressed { key_code: KeyCode::Up, ..} => Message::MoveUp,
		_ => Message::None,
	    },
	    _ => Message::None,
	};
	(Status::Captured, Some(message))
    }
}

