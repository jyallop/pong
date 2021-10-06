use iced::{ Point, canvas::Path, Rectangle, Vector };
use std::ops::Add;
use super::Drawable;

pub struct Ball {
    center: Point,
    radius: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
	Ball { center: Point::new(x, y), radius: radius }
    }

    pub fn move_ball(&mut self) -> () {
	self.center = self.center.add(Vector::new(0.01, 0.01));
    }

}

impl Drawable for Ball {
    fn draw(&self, frame_size: Rectangle<f32>) -> Path {
	Path::circle(Point::new(self.center.x * frame_size.width,
				self.center.y * frame_size.height),
		     self.radius * frame_size.width)
    }
}
