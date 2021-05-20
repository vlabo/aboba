use std::cell::Cell;
use std::rc::{Rc, Weak};
use ffmpeg_decoder;
use ffmpeg::{codec, filter, format, frame, media, rescale, Rescale};
use std::path::Path;

use rodio::Sink;

pub struct Player {

}

#[derive(Clone)]
pub struct Control {

}

impl Player {
    pub fn setup() -> Result<Self, ffmpeg::Error> {

        
        return Ok(Player {});
    }

    pub fn new_control(&self) -> Control {
        Control {
            
        }
    }

    pub fn set_file(&self, file: &String) -> Result<(), ffmpeg::Error> {

        let decoder = ffmpeg_decoder::Decoder::open(Path::new(file)).unwrap();

        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);

        sink.append(decoder);
        sink.play();
        sink.sleep_until_end();

        // let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        // let mut ictx = format::input(&file).unwrap();
        // let mut decoder;
        // let stream_index;
        // {
        //     let input = ictx
        //     .streams()
        //     .best(media::Type::Audio)
        //     .expect("could not find best audio stream");
        //     decoder = input.codec().decoder().audio()?;
        //     stream_index = input.index();
        // }
        // // let filter = filter::Graph::new();

        // let args = format!(
        //     "time_base={}:sample_rate={}:sample_fmt={}:channel_layout=0x{:x}",
        //     decoder.time_base(),
        //     decoder.rate(),
        //     decoder.format().name(),
        //     decoder.channel_layout().bits()
        // );

        // let mut decoded = frame::Audio::empty();

        // println!("{}", args);
        // let in_time_base = decoder.time_base();
        // for (stream, mut packet) in ictx.packets() {
        //     if stream.index() == stream_index {
        //         packet.rescale_ts(stream.time_base(), in_time_base);
        //         if let Ok(true) = decoder.decode(&packet, &mut decoded) {
        //             let timestamp = decoded.timestamp();
        //             decoded.set_pts(timestamp);
        //             println!("{}", timestamp.unwrap());
        //         }
        //     }
        // }

        Ok(())
    }
}

impl Control {
    pub fn play(&self) -> Result<(), ffmpeg::Error> {
        // if let Some(pipeline) = self.pipeline.upgrade() {
        //     let _ = pipeline.set_state(gst::State::Playing)?;
        // }
        return Ok(());
    }

    pub fn is_playing(&self) -> bool {
        let state = false;
        // if let Some(pipeline) = self.pipeline.upgrade() {
        //     let time = gst::ClockTime::from_seconds(10);
        //     let result = pipeline.get_state(time);
        //     state = result.1 == gst::State::Playing;
        // } else {
        //     state = false;
        // };
        return state;
    }

    pub fn pause(&self) -> Result<(), ffmpeg::Error> {
        // if let Some(pipeline) = self.pipeline.upgrade() {
        //     pipeline.set_state(gst::State::Paused)?;
        // };

        return Ok(());
    }

    pub fn get_position(&self) -> u64 {
        let position = 0;
        // if let Some(pipeline) = self.pipeline.upgrade() {
        //     let result = match pipeline.query_position::<gst::ClockTime>() {
        //         Some(pos) => pos,
        //         None => {
        //             eprintln!("Unable to retrieve current position...\r");
        //             return 0;
        //         }
        //     };
        //     position = result.seconds().unwrap();
        // } else {
        //     position = 0;
        // }

        return position;
    }

    #[allow(dead_code)]
    pub fn get_duration(&self) -> u64 {
        let duration = 0;
        // if let Some(pipeline) = self.pipeline.upgrade() {
        //     let result = match pipeline.query_duration::<gst::ClockTime>() {
        //         Some(pos) => pos,
        //         None => {
        //             eprintln!("Unable to retrieve duration...\r");
        //             return 0;
        //         }
        //     };
        //     duration = result.seconds().unwrap();
        // } else {
        //     duration = 0;
        // };

        return duration;
    }

    pub fn set_position(&self, position: u64) {
        // let time = gst::ClockTime::from_seconds(position);
        // if let Some(pipeline) = self.pipeline.upgrade() {
        //     let _ = pipeline.seek_simple(SeekFlags::FLUSH | SeekFlags::ACCURATE, time);
        // };
    }
}
