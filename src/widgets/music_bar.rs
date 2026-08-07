use iced::{Point, Size, mouse};
use iced::widget::canvas;
use iced::{Color, Renderer, Theme, Rectangle};

#[derive(Clone )]
pub struct MusicBar {
    pub top_left: Point,
    pub size: Size,
    pub color: Color
}

impl<Message> canvas::Program<Message> for MusicBar {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {

        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.fill_rectangle(self.top_left, self.size, Color::WHITE);


        vec![frame.into_geometry()]
    }
}