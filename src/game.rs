mod ball;
mod drawable;
mod paddle;
mod physics;

use ball::Ball;
use drawable::Drawable;
use iced::canvas::{Canvas, Frame, Program};
use iced::{
	canvas::{event::Status, Cursor, Event},
	executor,
	keyboard::KeyCode,
	time, Application, Command, Container, Element, Length,
};
use iced::{Color, Rectangle};
use paddle::Paddle;
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

impl Game {
	fn move_computer(&mut self) {
		if self.paddle_two.is_above(self.ball.get_top()) {
			self.paddle_two.slide(Direction::Up)
		} else if self.paddle_two.is_below(self.ball.get_bottom()) {
			self.paddle_two.slide(Direction::Down)
		}
	}
}

impl Application for Game {
	type Executor = executor::Default;

	type Message = Message;

	type Flags = ();

	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(
			Game {
				paddle_one: Paddle::new(0.02, 0.1, 0.02, 0.10),
				paddle_two: Paddle::new(0.96, 0.3, 0.02, 0.10),
				ball: Ball::new(0.5, 0.5, 0.01),
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		String::from("Pong")
	}

	fn update(
		&mut self,
		message: Self::Message,
		_clipboard: &mut iced::Clipboard,
	) -> iced::Command<Self::Message> {
		match message {
			Message::MoveDown => {
				self.paddle_one.slide(Direction::Down);
				Command::none()
			}
			Message::MoveUp => {
				self.paddle_one.slide(Direction::Up);
				Command::none()
			}
			Message::Tick(_) => {
				self.move_computer();
				self.ball.move_ball();
				if physics::check_collision(&self.ball, &self.paddle_one) {
					println!("collision one");
					self.ball.flip_x();
				};
				if physics::check_collision(&self.ball, &self.paddle_two) {
					println!("collision two");
					self.ball.flip_x();
				};
				if physics::check_wall_collision(&self.ball) {
					self.ball.flip_y();
				}
				Command::none()
			}
			_ => Command::none(),
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
		time::every(Duration::from_millis(10)).map(Message::Tick)
	}
}

impl Program<Message> for Game {
	fn draw(
		&self,
		bounds: iced::Rectangle,
		_cursor: iced::canvas::Cursor,
	) -> Vec<iced::canvas::Geometry> {
		// We prepare a new `Frname`
		let mut frame = Frame::new(bounds.size());

		frame.fill(&self.paddle_one.draw(bounds), Color::BLACK);

		frame.fill(&self.paddle_two.draw(bounds), Color::BLACK);

		frame.fill(&self.ball.draw(bounds), Color::BLACK);

		// Finally, we produce the geometry
		vec![frame.into_geometry()]
	}

	fn update(
		&mut self,
		event: Event,
		_bounds: Rectangle<f32>,
		_cursor: Cursor,
	) -> (Status, Option<Message>) {
		println!("Event: {:?}", event);
		let message = match event {
			Event::Keyboard(event) => match event {
				iced::keyboard::Event::KeyPressed {
					key_code: KeyCode::Down,
					..
				} => Message::MoveDown,
				iced::keyboard::Event::KeyPressed {
					key_code: KeyCode::Up,
					..
				} => Message::MoveUp,
				_ => Message::None,
			},
			_ => Message::None,
		};
		(Status::Captured, Some(message))
	}
}
