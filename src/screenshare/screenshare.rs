use crate::utils::utils::{
    bgr0_to_rgba, bgra_to_rgba, bgrx_to_rgba, rgb_to_rgba, rgbx_to_rgba, xbgr_to_rgba,
};
use scap::capturer::Capturer;
use scap::frame::Frame;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};


// Funzione migliorata per gestire lo screen sharing
pub fn start_screen_sharing(
    captures: Arc<Mutex<Option<Capturer>>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<Vec<u8>>>,
) {
    let mut start_time: u64 = 0;

    // Starts capture within a scope to release the lock after starting
    {
        let mut cap_lock = captures.lock().unwrap();
        if let Some(ref mut cap) = *cap_lock {
            cap.start_capture();
        }
    }

    // Capture loop
    while !stop_flag.load(Ordering::Relaxed) {
        // Retrieve frame at the beginning of the loop
        let frame_result = {
            let cap_lock = captures.lock().unwrap();
            cap_lock.as_ref().and_then(|cap| cap.get_next_frame().ok())
        };

        if let Some(frame) = frame_result {
            // Process frame data according to its format
            match frame {
                Frame::YUVFrame(_frame) => {
                    // Add specific processing for YUVFrame if needed
                }
                Frame::BGR0(frame) => {
                    sender
                        .send(bgr0_to_rgba(frame.data.clone()))
                        .expect("Failed to send BGR0 frame data");
                }
                Frame::RGB(frame) => {
                    if start_time == 0 {
                        start_time = frame.display_time;
                    }
                    sender
                        .send(rgb_to_rgba(frame.data.clone()))
                        .expect("Failed to send RGB frame data");
                }
                Frame::RGBx(frame) => {
                    sender
                        .send(rgbx_to_rgba(frame.data.clone()))
                        .expect("Failed to send RGBx frame data");
                }
                Frame::XBGR(frame) => {
                    sender
                        .send(xbgr_to_rgba(frame.data.clone()))
                        .expect("Failed to send XBGR frame data");
                }
                Frame::BGRx(frame) => {
                    sender
                        .send(bgrx_to_rgba(frame.data.clone()))
                        .expect("Failed to send BGRx frame data");
                }
                Frame::BGRA(frame) => {
                    if start_time == 0 {
                        start_time = frame.display_time;
                    }
                    sender
                        .send(bgra_to_rgba(frame.data.clone()))
                        .expect("Failed to send BGRA frame data");
                }
            }
        }
    }

}

pub fn stop_screen_sharing(capturer: Arc<Mutex<Option<Capturer>>>) {
    // Acquire the lock and stop capture if `capturer` is available
    let mut capturer_lock = capturer.lock().unwrap();
    if let Some(ref mut cap) = *capturer_lock {
        cap.stop_capture();
    }
}

/*  
    FUNZIONI UTILI?
pub fn check_issupported() -> bool {
    scap::is_supported()
}

pub fn check_haspermission() -> bool {
    scap::has_permission()
}

pub fn set_options() -> Options {
    //TODO we can set the options
    return Options {
        fps: 120,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::RGB,
        output_resolution: scap::capturer::Resolution::_1440p,

        ..Default::default()
    };
}

pub fn create_capture(options: Options) -> Capturer {
    Capturer::new(options)
}
*/
pub fn take_screenshot(captures: Arc<Mutex<Option<Capturer>>>) -> Vec<u8> {
    // Acquire the lock and start capture within a single scope
    {
        let mut cap_lock = captures.lock().unwrap();
        if let Some(ref mut cap) = *cap_lock {
            cap.start_capture();
        } else {
            return Vec::new(); // Return an empty Vec if no capturer is available
        }
    }
    
    // Retrieve the next frame
    let frame_result = {
        let cap_lock = captures.lock().unwrap();
        cap_lock.as_ref().and_then(|cap| cap.get_next_frame().ok())
    };

    let mut capturer_lock = captures.lock().unwrap();
    if let Some(ref mut cap) = *capturer_lock {
        cap.stop_capture();
    }

    // Handle the frame and return the corresponding data as Vec<u8>
    match frame_result {
        Some(Frame::YUVFrame(_)) => Vec::<u8>::new(), // Handle YUVFrame if needed
        Some(Frame::BGR0(frame)) => bgr0_to_rgba(frame.data),
        Some(Frame::RGB(frame)) => rgb_to_rgba(frame.data),
        Some(Frame::RGBx(frame)) => rgbx_to_rgba(frame.data),
        Some(Frame::XBGR(frame)) => xbgr_to_rgba(frame.data),
        Some(Frame::BGRx(frame)) => bgrx_to_rgba(frame.data),
        Some(Frame::BGRA(frame)) => bgra_to_rgba(frame.data),
        None => Vec::new(), // Return an empty Vec if no frame was captured
    }
}
