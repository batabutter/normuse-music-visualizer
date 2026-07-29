use normuse_music_visualizer::widgets::dashboard::{self, Dashboard};
use normuse_music_visualizer::types::conf::Conf;
use iced::{application, window, Element, Length};
use iced::widget::{column, container, grid, progress_bar, text, image};

struct Normuse {
    conf: Conf,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Normuse {
    
    pub fn new(conf: &Conf) -> Normuse {

        let conf_clone = conf.clone();

        Normuse {
            conf: conf_clone,
        }
    }
    
    pub fn update(&mut self, message: Message) {
        
    }
    
    pub fn view(&self) -> Element<'_, Message>{


        let dashboard = &self.conf.dashboard;
        let emblem = &self.conf.emblem;

        column![
            container(
                image("src\\assets\\default.png")
                .height(300)
                .width(300)
            )
            .width(iced::Fill)
            .height(iced::Fill)
            .center(iced::Fill),
            progress_bar(0.0..=100.0, 25.0)
        ]
        .into()
    }
    
}
pub fn main() -> iced::Result {

    let conf = Conf::default();
    let app_conf = conf.clone();

    let normuse = Normuse::new(&conf);

    iced::application(
        move || Normuse::new(&app_conf), 
        Normuse::update, 
        Normuse::view
    )
    .window(conf.window)
    .title("Normuse").run()
}