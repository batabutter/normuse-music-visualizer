use iced::widget::{Image, image};

const DEFAULT_FILEPATH: &str = "src\\assets\\chip.png";

pub struct Emblem {
    pub image_handle: image::Handle,
    pub filepath: String,
}

impl Emblem {

    pub fn new(filepath: &String) -> Emblem {
        let image = image::Handle::from_path(filepath);
        Emblem { image_handle: image, filepath: filepath.clone() }
    }

    pub fn default() -> Emblem {
        let image = image::Handle::from_path(DEFAULT_FILEPATH);
        Emblem { image_handle: image, filepath: DEFAULT_FILEPATH.to_string() }
    }

    pub fn view() {

    }
}

impl Clone for Emblem {
    fn clone(&self) -> Self {
        let image = image::Handle::from_path(DEFAULT_FILEPATH);
        Emblem { image_handle: image, filepath: self.filepath.clone()}
    }
}