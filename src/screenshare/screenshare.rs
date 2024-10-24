use std::io::Write;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use scap::capturer::{Area, Capturer, Options};
use scap::frame::Frame;
use crate::utils::utils::{bgr0_to_rgba, bgra_to_rgba, bgrx_to_rgba, rgb_to_rgba, rgbx_to_rgba, xbgr_to_rgba};

pub fn visualize_screen_sharing(){
        //start ffplay
        let mut child = Command::new("ffplay")
            .args(&[
                "-f", "rawvideo",         // Formato non compresso
                "-pixel_format", "rgb24",  // Formato dei pixel: BGR con 0 per il canale alfa
                "-video_size", "1440x900", // Risoluzione del video (modifica secondo necessità)
                "-framerate", "120",       // Framerate (modifica secondo necessità)
                "-"
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .expect("Errore nell'avviare ffplay. Assicurati che ffplay sia installato e nel PATH.");

        let mut out = child;
        //TODO complete
    }


// Funzione migliorata per gestire lo screen sharing
pub fn start_screen_sharing(captures: Arc<Mutex<Capturer>>, stop_flag: Arc<AtomicBool>,sender:Arc<Sender<Vec<u8>>>) {
    let mut start_time: u64 = 0;
    // Acquisisce il lock per l'intero processo di cattura e inizia la cattura
    {
        let mut cap = captures.lock().unwrap();
        cap.start_capture();
    }


    while !stop_flag.load(Ordering::Relaxed) {
        // Recupera il frame all'inizio del loop con un solo lock
        let frame = {
            let mut cap = captures.lock().unwrap();
            cap.get_next_frame().expect("Error")
        };

        // Blocca l'output una sola volta per frame e processa i dati
        match frame {
            Frame::YUVFrame(frame) => {

            }
            Frame::BGR0(frame) => {
                sender.send(bgr0_to_rgba(frame.data.clone())).expect("TODO: panic message");

            }
            Frame::RGB(frame) => {
                if start_time == 0 {
                    start_time = frame.display_time;
                }
                sender.send(rgb_to_rgba(frame.data.clone())).expect("TODO: panic message");
            }
            Frame::RGBx(frame) => {
                sender.send(rgbx_to_rgba(frame.data.clone())).expect("TODO: panic message");
            }
            Frame::XBGR(frame) => {
                sender.send(xbgr_to_rgba(frame.data.clone())).expect("TODO: panic message");
            }
            Frame::BGRx(frame) => {
                sender.send(bgrx_to_rgba(frame.data.clone())).expect("TODO: panic message");
            }
            Frame::BGRA(frame) => {
                if start_time == 0 {
                    start_time = frame.display_time;
                }
                sender.send(bgra_to_rgba(frame.data.clone())).expect("TODO: panic message");
            }
        }
    }
    println!("sono fuori dal while tarari tarata")
}


pub fn stop_screen_sharing(mut capturer: Arc<Mutex<Capturer>>){
    let mut capturer = capturer.lock().unwrap();
        // Stop Capture
        capturer.stop_capture();
    }

    pub fn check_issupported()->bool{
        scap::is_supported()
    }

    pub fn check_haspermission()->bool{
        scap::has_permission()
    }

    pub fn set_options()->Options{
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

    pub fn create_capture(options: Options)->Capturer{
        Capturer::new(options)
    }

    pub fn take_screenshot(captures: Arc<Mutex<Capturer>>)->Vec<u8>{
        {
            let mut cap = captures.lock().unwrap();
            cap.start_capture();
        }
        let frame = {
            let mut cap = captures.lock().unwrap();
            cap.get_next_frame().expect("Error")
        };

        match frame {
            Frame::YUVFrame(frame) => {
               Vec::<u8>::new()
            }
            Frame::BGR0(frame) => {
                frame.data
            }
            Frame::RGB(frame) => {
                frame.data
            }
            Frame::RGBx(frame) => {
                frame.data
            }
            Frame::XBGR(frame) => {
                frame.data
            }
            Frame::BGRx(frame) => {
                frame.data
            }
            Frame::BGRA(frame) => {
                frame.data
            }
        }

    }


