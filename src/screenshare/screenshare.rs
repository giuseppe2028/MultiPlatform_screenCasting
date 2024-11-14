use crate::utils::utils::{
    bgr0_to_rgba, bgra_to_rgba, bgrx_to_rgba, rgb_to_rgba, rgbx_to_rgba, xbgr_to_rgba,
};

use xcap::image::{ImageBuffer, Rgba};
use xcap::{Monitor, XCapError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Funzione migliorata per gestire lo screen sharing
pub fn start_screen_sharing(
    monitor: Arc<Mutex<Monitor>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<Vec<u8>>>
) {
    // Capture loop
    while !stop_flag.load(Ordering::Relaxed) {
        // Cattura il frame dallo schermo
        let frame_result = {
            let mon_lock = monitor.lock().unwrap();
            mon_lock.capture_image()
        };
        match frame_result {
            Ok(frame) => {
                // Estrai i dati del buffer in formato raw
                let raw_data = frame.into_raw(); // Questo metodo estrae i dati del buffer

                // Invia i dati attraverso il canale
                if let Err(send_err) = sender.send(raw_data) {
                    eprintln!("Errore nell'invio dei dati del frame: {:?}", send_err);
                }
            }
            Err(e) => {
                // Gestione dell'errore: registrare o stampare l'errore
                eprintln!("Errore durante la cattura dello schermo: {:?}", e);
            }
        }
    }
}

/*pub fn stop_screen_sharing(capturer: Arc<Mutex<Option<Capturer>>>) {
    // Acquire the lock and stop capture if `capturer` is available
    let mut capturer_lock = capturer.lock().unwrap();
    if let Some(ref mut cap) = *capturer_lock {
        cap.stop_capture();
    }
}*/

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
pub fn take_screenshot(monitor: Arc<Mutex<Monitor>>) -> Vec<u8> {
    let frame_result = {
        let mon_lock = monitor.lock().unwrap();
        mon_lock.capture_image()
    };

    match frame_result {
        Ok(frame) => {
            // Estrai i dati del buffer in formato raw
            let raw_data = frame.into_raw(); // Questo metodo estrae i dati del buffer

            return raw_data
        }
        Err(e) => {
            // Gestione dell'errore: registrare o stampare l'errore
            eprintln!("Errore durante la cattura dello schermo: {:?}", e);
        }
    }
        return vec![];
}
