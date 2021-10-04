mod paddle;
mod ball;

use iced::{time, Application, Command, Container, Element, Length, Point, Size, canvas::{Cursor, Event, event::{self, Status}}, executor, keyboard::KeyCode};
use iced::canvas::{self, Canvas, Frame, Geometry, Path, Program};
use iced::{Color, Rectangle};
use paddle::Paddle;
use ball::Ball;
use std::time::{Duration, Instant};

use self::paddle::Direction;

pub struct Game {
    paddle_one: Paddle,
    paddle_two: Paddle,
    ball: Ball,
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
	(Game { paddle_one: Paddle::new(0.02, 0.1, 0.02, 0.10),
		paddle_two: Paddle::new(0.96, 0.1, 0.02, 0.10),
		ball: Ball::new(0.5, 0.5, 0.01) }, Command::none())
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
	    Message::None => Command::none(),
	    Message::MoveDown => { self.paddle_one.slide(Direction::Down); Command::none() },
	    Message::MoveUp => { self.paddle_one.slide(Direction::Up); Command::none() },
	    Message::Tick(_) => { self.ball.move_ball(); Command::none() },
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

	let paddle_one = self.paddle_one.draw(bounds);
	let paddle_two = self.paddle_two.draw(bounds);
	let ball = self.ball.draw(bounds);

	frame.fill(&paddle_one, Color::BLACK);
	frame.fill(&paddle_two, Color::BLACK);
	frame.fill(&ball, Color::BLACK);

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

