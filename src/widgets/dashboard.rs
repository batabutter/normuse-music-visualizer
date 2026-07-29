use iced::Size;

const DEFAULT_WIDTH: f32 = 300.0f32;
const DEFAULT_HEIGHT: f32 = 300.0f32;


pub struct Dashboard {
    pub width: f32,
    pub height: f32,
}

impl Dashboard {
    pub fn new(size: &Size<f32>) -> Dashboard {
        Dashboard { width: size.width, height: size.height }
    }

    pub fn default() -> Dashboard {
        Dashboard { width: DEFAULT_WIDTH, height: DEFAULT_HEIGHT }
    }
}

impl Clone for Dashboard {
    fn clone(&self) -> Self {
        Dashboard { width: self.width, height: self.height }   
    }
}