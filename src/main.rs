use std::thread;
use std::time::Duration;

use normuse_music_visualizer::types::conf::Conf;
use iced::{Element, Subscription, event, window,};
use iced::widget::{column, stack, container, progress_bar, image, canvas};
use normuse_music_visualizer::widgets::visualizer::Visualizer;

const BUFFER_SIZE:usize = 1024;

struct Normuse {
    conf: Conf,
    visualizer: Visualizer
}

#[derive(Debug, Clone)]
enum Message {
    WindowResized(iced::Size),
    Tick,
}

impl Normuse {

    pub fn subscription(&self) -> Subscription<Message> {
        //f: fn(Event, event::Status, window::Id) -> Option<Message>,
        let window_listener = event::listen_with(|event, _, _| {
            match event {
                event::Event::Window(window::Event::Resized(size)) => {
                    Some(Message::WindowResized(size))
                },
                _ => None
            }
        });

        let frame_listener = iced::time::every(Duration::from_secs_f32(1.0 / 60.0 )).map( |_| {
          Message::Tick
        });

        Subscription::batch([
            window_listener,
            frame_listener
        ])
    }

    pub fn new(conf: &Conf) -> Normuse {

        let conf_clone = conf.clone();
        let bar_width = 5.0f32;
        let num_bars = BUFFER_SIZE as f32 / bar_width;
        let visualizer = Visualizer::new(num_bars, "src/assets/test.wav");
        

        Normuse {
            conf: conf_clone, visualizer: visualizer
        }
    }
    
    pub fn update(&mut self, message: Message) {
        match message {
            Message::WindowResized(size) => {
                println!("Window size is now {}, {}", size.width, size.height);
            },
            Message::Tick => {
                if !self.visualizer.player.empty() {
                }
                self.visualizer.update();
            }
        }
    }
    
    pub fn view(&self) -> Element<'_, Message>{

        let dashboard = &self.conf.dashboard;
        let emblem = &self.conf.emblem;

        stack![
            canvas(&self.visualizer.display).width(iced::Fill).height(iced::Fill),
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
    .subscription(Normuse::subscription)
    .window(conf.window)
    .title("Normuse").run()
}