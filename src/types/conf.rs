use iced::window::{self, Settings};
use iced::Size;
use crate::widgets::{dashboard::Dashboard, emblem::Emblem, visualizer::{self, Visualizer}};

const BUFFER_SIZE: u32 = 1024;

pub struct Conf {
    pub window: Settings,
    pub dashboard: Dashboard,
    pub emblem: Emblem,
    pub visualizer: Visualizer,
}

impl Conf {

    /// Default configuration settings 
    pub fn default() -> Conf {

        let bar_width = 5.0f32;

        let num_bars = BUFFER_SIZE / bar_width as u32;

        let window = Settings::default();
        let dashboard = Dashboard::default();
        let emblem = Emblem::default();
        let visualizer = Visualizer::new(num_bars, bar_width);

        Conf {
            window: window,
            dashboard: dashboard,
            emblem: emblem,
            visualizer: visualizer
        }
    }
}

impl Clone for Conf {
    fn clone(&self) -> Self {
        Conf {
            dashboard: self.dashboard.clone(),
            window: self.window.clone(),
            emblem: self.emblem.clone(),
            visualizer: self.visualizer.clone()
        }
    }
}