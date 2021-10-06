use iced::Rectangle;
use iced::canvas::Path;

pub trait Drawable {
    fn draw(&self, frame_size: Rectangle<f32>) -> Path;
}
