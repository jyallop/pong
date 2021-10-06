use iced::{Point, Rectangle, Size, canvas::Path};
use super::Drawable;

pub struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub enum Direction {
    Up,
    Down,
}

impl Paddle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
	Paddle { x, y, width, height}
    }

    pub fn slide(&mut self, direction: Direction) -> () {
	match direction {
	    Direction::Up if self.y > 0.0 => self.y = self.y - 0.01,
	    Direction::Down if self.y < 0.9 => self.y = self.y + 0.01,
	    _ => ()
	}
    }
}

impl Drawable for Paddle {
    fn draw(&self, frame_size: Rectangle<f32>) -> Path {
	Path::rectangle(Point { x: self.x * frame_size.width,
				y: self.y * frame_size.height},
			Size { width: self.width * frame_size.width,
			       height: self.height * frame_size.height})
    }

}
