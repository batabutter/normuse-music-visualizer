use std::{thread, vec};
use std::sync::mpsc;

use iced::mouse;
use iced::widget::{Canvas, canvas};
use iced::{Color, Rectangle, Renderer, Theme};
use rodio::{MixerDeviceSink, Player, buffer};

use std::io::BufReader;
use std::fs::File;
use rodio::{Decoder, source::Source};

use crate::utils::audio_streamer::AudioStreamer;

const BUFFER_SIZE:usize = 2048;

pub struct VisualizerDisplay { 
    pub spacing: f32,
    pub num_bars: f32,
    pub data: Vec<f32>,
}

impl<Message> canvas::Program<Message> for VisualizerDisplay {
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
        let bar_width:f32 = bounds.width / self.num_bars;

        for i in 0..self.num_bars as u32{
            frame.fill_rectangle(
                iced::Point { x: i as f32 * bar_width, y: bounds.height / 2.0f32},
                iced::Size{ width: bar_width, height: 1000.0f32 * self.data[i as usize]}, 
                if i % 2 == 0 { Color::BLACK } else { Color::WHITE });
        }
        vec![frame.into_geometry()]
    }
}


pub struct Visualizer {
    pub player: Player,
    pub display: VisualizerDisplay,
    handle: MixerDeviceSink,
    tx: mpsc::SyncSender<f32>,
    decoder: Decoder<BufReader<File>>,
    channels: u16,
    current_frame: u32,
}
//"src/assets/test.wav"
impl Visualizer {
        pub fn new(num_bars: f32, filepath: &str) -> Self {

        let file = File::open(filepath).unwrap();
        let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
    
        let player = rodio::Player::connect_new(&handle.mixer());

        let decoder = Decoder::try_from(file).unwrap();
        let channels = decoder.channels().get();
        let sample_rate=  decoder.sample_rate().get();

        let (tx, rx) = mpsc::sync_channel(BUFFER_SIZE);
        
        let audio_streamer = AudioStreamer::new(rx, sample_rate, channels);
        player.append(audio_streamer);

        let data = vec![0.0f32; BUFFER_SIZE];
        let display = VisualizerDisplay { num_bars: num_bars, spacing: 0.0f32, data: data};

        Visualizer { player: player, display: display, handle: handle, tx: tx, decoder: decoder, channels: channels, current_frame: 0 }
    }

    pub fn queue_samples(&mut self, buf: &mut Vec<f32>) {
        for i in 0..buf.len() {
            let sample = self.decoder.next().unwrap();
            buf[i] = sample;
            if self.tx.try_send(sample).is_err() {
                break;
            }
        }
    }

    pub fn update(&mut self) {
        let mut buf:Vec<f32> = vec![0.0f32; BUFFER_SIZE];
        self.queue_samples(&mut buf);
        
        // Need to select every num channelth sample 
        buf = buf.iter().enumerate().filter(|(i,_)| * i as u16 % self.channels == 0).map(|(_,val)| *val).collect();
        
        self.display.data = buf;

    }
}