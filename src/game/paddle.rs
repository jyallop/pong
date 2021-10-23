use iced::{Point, Rectangle, Size, canvas::Path};
use super::Drawable;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
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
	    Direction::Down => self.y = min(0.9, self.y + 0.02),
	    Direction::Up => self.y = max(0.0, self.y - 0.02),
	    _ => ()
	}
    }

    pub fn is_above(&self, other_y: f32) -> bool {
	let top = self.y + self.height;
	if top > other_y {
	    true
	} else {
	    false
	}
    }

    pub fn is_below(&self, other_y: f32) -> bool {
	if self.y < other_y {
	    true
	} else {
	    false
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

fn min(x: f32, y: f32) -> f32 {
    if x > y {
	y
    } else {
	x
    }
}

fn max(x: f32, y: f32) -> f32 {
    if x < y {
	y
    } else {
	x
    }
}
