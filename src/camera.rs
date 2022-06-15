use std::fmt::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use uvc::ActiveStream;

pub struct Camera {
    stream_format: uvc::StreamFormat,
}

impl Camera {
    pub fn new(stream_format: uvc::StreamFormat) -> Self {
        Self { stream_format }
    }

    pub fn run(self) -> ActiveStream<'static, Arc<AtomicUsize>> {
        let ctx = uvc::Context::new().expect("Could not get context");

        let dev = ctx
            .find_device(None, None, None)
            .expect("Could not find device");

        // let mut _devices = ctx.devices().expect("Could not enumerate devices");

        // println!("Devices: {:?}", _devices);

        let dev_handle = dev.open().expect("Could not open device");

        let format = self.stream_format;

        let mut stream_handle = dev_handle
            .get_stream_handle_with_format(format)
            .expect("Could not open stream with defined format");

        let counter = Arc::new(AtomicUsize::new(0));

        let stream = stream_handle
            .start_stream(
                |_frame, count| {
                    count.fetch_add(1, Ordering::SeqCst);
                },
                counter.clone(),
            )
            .expect("Could not start Stream");

        stream
    }
}
