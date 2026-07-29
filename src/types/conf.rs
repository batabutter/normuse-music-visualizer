use iced::window::Settings;
use crate::widgets::{dashboard::Dashboard, emblem::Emblem,};

pub struct Conf {
    pub window: Settings,
    pub dashboard: Dashboard,
    pub emblem: Emblem
}

impl Conf {

    /// Default configuration settings 
    pub fn default() -> Conf {

        let window = Settings::default();
        let dashboard = Dashboard::default();
        let emblem = Emblem::default();

        Conf {
            window: window,
            dashboard: dashboard,
            emblem: emblem
        }
    }
}

impl Clone for Conf {
    fn clone(&self) -> Self {
        Conf {
            dashboard: self.dashboard.clone(),
            window: self.window.clone(),
            emblem: self.emblem.clone()
        }
    }
}