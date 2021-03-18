use super::{Open, Sink};
use gst::prelude::*;
use gst::*;
use std::{io, thread};
use zerocopy::*;

#[allow(dead_code)]
pub struct GstreamerSink {
    pipeline: gst::Pipeline,
    appsrc: gst_app::AppSrc,
}

impl Open for GstreamerSink {
    fn open(device: Option<String>) -> GstreamerSink {
        gst::init().expect("Failed to init gstreamer!");
        let pipeline_str_preamble = r#"appsrc caps="audio/x-raw,format=S16LE,layout=interleaved,channels=2,rate=44100" block=true max-bytes=4096 name=appsrc0 "#;
        let pipeline_str_rest = r#" ! audioconvert ! autoaudiosink"#;
        let pipeline_str: String = match device {
            Some(x) => format!("{}{}", pipeline_str_preamble, x),
            None => format!("{}{}", pipeline_str_preamble, pipeline_str_rest),
        };
        info!("Pipeline: {}", pipeline_str);

        gst::init().unwrap();
        let pipelinee = gst::parse_launch(&*pipeline_str).expect("Couldn't launch pipeline; likely a GStreamer issue or an error in the pipeline string you specified in the 'device' argument to librespot.");
        let pipeline = pipelinee
            .dynamic_cast::<gst::Pipeline>()
            .expect("Couldn't cast pipeline element at runtime!");
        let bus = pipeline.get_bus().expect("Couldn't get bus from pipeline");
        let mainloop = glib::MainLoop::new(None, false);
        let appsrce: gst::Element = pipeline
            .get_by_name("appsrc0")
            .expect("Couldn't get appsrc from pipeline");
        let appsrc: gst_app::AppSrc = appsrce
            .dynamic_cast::<gst_app::AppSrc>()
            .expect("Couldn't cast AppSrc element at runtime!");

        thread::spawn(move || {
            let thread_mainloop = mainloop;
            let watch_mainloop = thread_mainloop.clone();
            bus.add_watch(move |_, msg| {
                match msg.view() {
                    MessageView::Eos(..) => watch_mainloop.quit(),
                    MessageView::Error(err) => {
                        println!(
                            "Error from {:?}: {} ({:?})",
                            err.get_src().map(|s| s.get_path_string()),
                            err.get_error(),
                            err.get_debug()
                        );
                        watch_mainloop.quit();
                    }
                    _ => (),
                };

                glib::Continue(true)
            })
            .expect("Failed to add bus watch");
            thread_mainloop.run();
        });

        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");

        GstreamerSink {
            pipeline: pipeline,
            appsrc: appsrc,
        }
    }
}

impl Sink for GstreamerSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let data_bytes: &[u8] = data.as_bytes();

        let mut buffer =
            gst::Buffer::with_size(data_bytes.len())
            .expect("Failed to allocate Buffer");
        buffer
            .make_mut()
            .copy_from_slice(0, data_bytes)
            .expect("Failed to copy from slice");
        self.appsrc.push_buffer(buffer).expect("Failed to push buffer");

        Ok(())
    }
}
