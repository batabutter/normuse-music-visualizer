use iced::{Point, Size, mouse};
use iced::widget::canvas;
use iced::{Color, Rectangle, Renderer, Theme};

use crate::widgets::music_bar::{self, MusicBar};
pub struct Visualizer {
    pub spacing: u32,
    pub num_bars: u32,
    pub music_bars: Vec<MusicBar>,
}

impl Visualizer {

    pub fn new(num_bars: u32, rectangle_width: f32) -> Self {
        let size: Size<f32> = Size { width: rectangle_width, height: 50.0f32}; 
        let mut music_bars=  Vec::<MusicBar>::new();
        for i in 0..num_bars {
            let curr_point = Point { x: i as f32 * rectangle_width, y: 0.0f32 };
            music_bars.push(MusicBar { top_left: curr_point, size: size, color: Color::WHITE });
        }


        Visualizer { num_bars: num_bars, spacing: 0, music_bars: music_bars}
    }
}

impl Clone for Visualizer {
    fn clone(&self) -> Self {
        Visualizer { num_bars: self.num_bars, spacing: self.spacing.clone(), music_bars: self.music_bars.clone()}
    }
}

impl<Message> canvas::Program<Message> for Visualizer {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for (i,bar) in self.music_bars.iter().enumerate() {
            frame.fill_rectangle(bar.top_left, bar.size, if i % 2 == 0 { Color::BLACK } else { Color::WHITE });
        }


        vec![frame.into_geometry() ]
    }
}