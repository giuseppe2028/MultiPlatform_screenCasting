use scap::{
    capturer::{Capturer, Options},
    targets::Display,
    Target,
};

use crate::screenshare::screenshare::{start_screen_sharing, take_screenshot};
use crate::screenshare::screenshare::stop_screen_sharing;
use iced::keyboard::KeyCode::M;
use scap::frame::Frame;
use std::process::{Child, ChildStdin, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, Thread, ThreadId};
use scap::capturer::{Area, Point, Size};
use scap::targets::get_target_dimensions;
use url::quirks::origin;

pub struct AppController {
    pub capturer: Arc<Mutex<Option<Capturer>>>,
    pub option: Options,
    pub streaming_handle: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<Vec<u8>>>,
    pub is_just_stopped:bool
}

impl AppController {
    // Costruttore per AppController
    pub fn new(option: Options, sender: Sender<Vec<u8>>) -> Self {
        AppController {
            capturer: Arc::new(Mutex::new(None)),
            option,
            streaming_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender),
            is_just_stopped:false
        }
    }

    // Funzione che avvia la condivisione dello schermo
    pub fn start_sharing(&mut self) {
        self.stop_flag.store(false, Ordering::Relaxed);
        println!("Lo schermo selezionato: {:?}", self.option.target);
        self.capturer = Arc::new(Mutex::new(Some(Capturer::new(self.option.clone()))));
        let capturer = Arc::clone(&self.capturer);
        let stop_flag = Arc::clone(&self.stop_flag);
        let send = self.sender.clone();
        // Crea un nuovo thread per lo screen sharing
        println!("options {:?}", self.option);
        let handle = Some(thread::spawn(move || {
            // Passiamo stdin e altri dati al thread
            start_screen_sharing(capturer, stop_flag, send);
        }));
        self.set_handle(handle.unwrap());
    }

    pub fn set_options(&mut self, options: Options) {
        self.option = options;
    }

    pub fn set_handle(&mut self, handle: JoinHandle<()>) {
        self.streaming_handle = Some(handle);
    }

    pub fn set_display(&mut self, display: Display) {
        self.option.target = Some(Target::Display(display.clone()));
        self.option.output_resolution =
            scap::capturer::Resolution::get_resolution(display.get_width());
    }

    pub fn set_coordinates(&mut self, x: f64, y: f64,start_x:f64,start_y:f64) {

        self.option.crop_area = Some(
            Area {
                origin: Point { x: start_x, y: start_y },
                size: Size { width:x, height:y},
            }
        );
    }


    pub fn get_available_displays(&self) -> Vec<scap::targets::Display> {
        let displays: Vec<scap::targets::Display> = scap::get_all_targets()
            .into_iter()
            .filter_map(|target| {
                if let Target::Display(display) = target {
                    Some(display) // Return the Display if found
                } else {
                    None // Ignore all other types
                }
            })
            .collect();

        return displays;
    }

    pub fn stop_recording(&mut self) {
        // Imposta il flag per fermare il thread
        self.stop_flag.store(true, Ordering::Relaxed);

        // Aspetta che il thread di streaming termini (se esiste)
        if let Some(handle) = self.streaming_handle.take() {
            handle
                .join()
                .expect("Errore nella terminazione del thread di streaming");
        }
        println!("Ciao! Il thread di streaso.");
        stop_screen_sharing(self.capturer.clone());
        println!("Ciao! Il thread di streaming è stato chiuso.");
    }
    pub fn take_screenshot(&mut self)->Vec<u8>{

        take_screenshot(self.capturer.clone())

    }
    pub fn set_is_just_stopped(&mut self,value:bool){
        self.is_just_stopped = value;
    }
    pub fn get_measures(&self) -> (u32, u32) {
        match self.option.output_resolution {
            scap::capturer::Resolution::_480p => (640, 480),       // 480p: 640x480
            scap::capturer::Resolution::_720p => (1280, 720),      // 720p: 1280x720
            scap::capturer::Resolution::_1080p => (1920, 1080),    // 1080p: 1920x1080
            scap::capturer::Resolution::_1440p => (1440 , 900),    // 1440p: 2560x1440
            scap::capturer::Resolution::_2160p => (3840, 2160),    // 2160p: 3840x2160
            scap::capturer::Resolution::_4320p => (7680, 4320),    // 4320p: 7680x4320
            scap::capturer::Resolution::Captured => {
                (1920, 1080)
            },
        }
    }
}
