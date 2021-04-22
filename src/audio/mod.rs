use gst::gst_element_error;
use gst::gst_element_warning;
use gst::prelude::*;

use gst::{SeekFlags};

use anyhow::Error;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

pub struct Player {
    pipeline: gst::Pipeline,
    src: gst::Element,
}

#[derive(Clone)]
pub struct Control {
    pipeline: glib::WeakRef<gst::Pipeline>,
}

impl Player {
    pub fn setup() -> Result<Self, Error> {
        gst::init()?;

        let pipeline = gst::Pipeline::new(None);
        let src =
            gst::ElementFactory::make("filesrc", None).map_err(|_| MissingElement("filesrc"))?;
        let decodebin = gst::ElementFactory::make("decodebin", None)
            .map_err(|_| MissingElement("decodebin"))?;


        pipeline.add_many(&[&src, &decodebin])?;
        gst::Element::link_many(&[&src, &decodebin])?;


        let pipeline_weak = pipeline.downgrade();

        decodebin.connect_pad_added(move |dbin, src_pad| {
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return,
            };

            let (is_audio, is_video) = {
                let media_type = src_pad.get_current_caps().and_then(|caps| {
                    caps.get_structure(0).map(|s| {
                        let name = s.get_name();
                        (name.starts_with("audio/"), name.starts_with("video/"))
                    })
                });

                match media_type {
                    None => {
                        gst_element_warning!(
                            dbin,
                            gst::CoreError::Negotiation,
                            ("Failed to get media type from pad {}", src_pad.get_name())
                        );

                        return;
                    }
                    Some(media_type) => media_type,
                }
            };

            let insert_sink = |is_audio, _| -> Result<(), Error> {
                if is_audio {
                    let queue = gst::ElementFactory::make("queue", None)
                        .map_err(|_| MissingElement("queue"))?;
                    let convert = gst::ElementFactory::make("audioconvert", None)
                        .map_err(|_| MissingElement("audioconvert"))?;
                    let resample = gst::ElementFactory::make("audioresample", None)
                        .map_err(|_| MissingElement("audioresample"))?;
                    let sink = gst::ElementFactory::make("autoaudiosink", None)
                        .map_err(|_| MissingElement("autoaudiosink"))?;

                    let elements = &[&queue, &convert, &resample, &sink];
                    pipeline.add_many(elements)?;
                    gst::Element::link_many(elements)?;

                    for e in elements {
                        e.sync_state_with_parent()?;
                    }

                    let sink_pad = queue.get_static_pad("sink").expect("queue has no sinkpad");
                    src_pad.link(&sink_pad)?;
                }

                Ok(())
            };

            if let Err(err) = insert_sink(is_audio, is_video) {
                gst_element_error!(
                    dbin,
                    gst::LibraryError::Failed,
                    ("Failed to insert sink"),
                    ["{}", err]
                );
            }
        });

        // pipeline.set_state(gst::State::Paused)?;
        let bus = pipeline
            .get_bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        // let pipeline_weak = pipeline.downgrade();
        bus.add_watch(move |_, msg| {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => {},
                MessageView::Error(err) => {
                    // if let Some(pipeline) = pipeline_weak.upgrade() {
                    //     pipeline.set_state(gst::State::Null).unwrap();
                    // }
                    println!("{}", ErrorMessage {
                        src: msg
                            .get_src()
                            .map(|s| String::from(s.get_path_string()))
                            .unwrap_or_else(|| String::from("None")),
                        error: err.get_error().to_string(),
                        debug: err.get_debug(),
                        source: err.get_error(),
                    });
                }
                MessageView::StateChanged(s) => {
                    println!(
                        "State changed from {:?}: {:?} -> {:?} ({:?})",
                        s.get_src().map(|s| s.get_path_string()),
                        s.get_old(),
                        s.get_current(),
                        s.get_pending()
                    );
                }
                MessageView::Toc(msg_toc) => {
                    let (toc, updated) = msg_toc.get_toc();
                    println!(
                        "\nReceived toc: {:?} - updated: {}",
                        toc.get_scope(),
                        updated
                    );
                    // Get a list of tags that are ToC specific.
                    if let Some(tags) = toc.get_tags() {
                        println!("- tags: {}", tags.to_string());
                    }
                }
                _ => (),
            }
            glib::Continue(true)
        }).expect("Failed to add bus watch");
        return Ok(Player { pipeline, src });
    }

    pub fn new_control(&self) -> Control {
        Control { pipeline: self.pipeline.downgrade() }
    }

    pub fn set_file(&self, file: &String)  {
        self.src.set_property("location", file).unwrap();
    }
}

impl Control {

    pub fn play(&self) -> Result<(), Error> {
        if let Some(pipeline) = self.pipeline.upgrade() {
            let _ = pipeline.set_state(gst::State::Playing)?;
        }
        return Ok(());
    }

    pub fn is_playing(&self) -> bool {
        let state;
        if let Some(pipeline) = self.pipeline.upgrade() {
            let time = gst::ClockTime::from_seconds(10);
            let result = pipeline.get_state(time);
            state = result.1 == gst::State::Playing;
        } else {
            state = false;
        };
        return state;
    }

    pub fn pause(&self) -> Result<(), Error> {
        if let Some(pipeline) = self.pipeline.upgrade() {
            pipeline.set_state(gst::State::Paused)?;
        };

        return Ok(());
    }

    pub fn get_position(&self) -> u64 {
        let position;
        if let Some(pipeline) = self.pipeline.upgrade() {
            let result = match pipeline.query_position::<gst::ClockTime>() {
                Some(pos) => pos,
                None => {
                    eprintln!("Unable to retrieve current position...\r");
                    return 0;
                }
            };
            position = result.seconds().unwrap();
        } else {
            position = 0;
        }

        return position;
    }

    #[allow(dead_code)]
    pub fn get_duration(&self) -> u64 {
        let duration;
        if let Some(pipeline) = self.pipeline.upgrade() {
            let result = match pipeline.query_duration::<gst::ClockTime>() {
                Some(pos) => pos,
                None => {
                    eprintln!("Unable to retrieve duration...\r");
                    return 0;
                }
            };
            duration = result.seconds().unwrap();
        } else {
            duration = 0;
        };

        return duration;
    }

    pub fn set_position(&self, position: u64) {
        let time = gst::ClockTime::from_seconds(position);
        if let Some(pipeline) = self.pipeline.upgrade() {
            let _ = pipeline.seek_simple(SeekFlags::FLUSH | SeekFlags::ACCURATE, time);
        };
    }

}
