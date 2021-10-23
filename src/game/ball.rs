use iced::{ Point, canvas::Path, Rectangle, Vector };
use std::ops::Add;
use std::ops::Neg;
use super::Drawable;

pub struct Ball {
    pub center: Point,
    pub radius: f32,
    pub velocity: Vector,
}

impl Ball {
    pub fn new_ball(x: f32, y: f32, radius: f32, velocity_x: f32, velocity_y: f32) -> Self {
	Ball { center: Point::new(x, y),
	       radius: radius,
	       velocity: Vector::new(velocity_x, velocity_y)
	}
    }

    pub fn new(x: f32, y: f32, radius: f32) -> Self {
	Ball::new_ball(x, y, radius, 0.01, 0.01)
    }

    pub fn move_ball(&mut self) -> () {
	self.center = self.center.add(self.velocity);
    }

    pub fn get_top(&self) -> f32 {
	self.center.y + self.radius
    }

    pub fn get_bottom(&self) -> f32 {
	self.center.y - self.radius
    }

    pub fn flip_x(&mut self) {
	self.velocity.x = Neg::neg(self.velocity.x)
    }

    pub fn flip_y(&mut self) {
	self.velocity.y = Neg::neg(self.velocity.y)
    }
}

impl Drawable for Ball {
    fn draw(&self, frame_size: Rectangle<f32>) -> Path {
	Path::circle(Point::new(self.center.x * frame_size.width,
				self.center.y * frame_size.height),
		     self.radius * frame_size.width)
    }
}
