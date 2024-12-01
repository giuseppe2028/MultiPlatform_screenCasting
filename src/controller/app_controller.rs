use futures::lock;
use xcap::Monitor;

use crate::screenshare::screenshare::{start_screen_sharing, take_screenshot};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use xcap::image::RgbaImage;
use crate::model::Shortcut::{from_key_code_to_string, from_str_to_key_code, ShortcutController};

pub struct AppController {
    pub monitor_chosen: Arc<Mutex<Monitor>>,
    pub streaming_handle: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<RgbaImage>>,
    pub is_just_stopped: bool,
    pub shortcut:ShortcutController
}

impl AppController {
    // Costruttore per AppController
    pub fn new(monitor: Monitor, sender: Sender<RgbaImage>) -> Self {
        AppController {
            monitor_chosen: Arc::new(Mutex::new(monitor)),
            streaming_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender),
            is_just_stopped: false,
            shortcut: ShortcutController::new_from_file(),
        }
    }

    // Funzione che avvia la condivisione dello schermo
    pub fn start_sharing(&mut self) {
        self.stop_flag.store(false, Ordering::Relaxed);

        /*let mut capturer_guard = self.capturer.lock().unwrap();
        if capturer_guard.is_none() {
            self.capturer = Arc::new(Mutex::new(Some(Capturer::new(self.option.clone()))));
        }
        */

        let monitor = self.monitor_chosen.clone();
        let stop_flag = Arc::clone(&self.stop_flag);
        let send = self.sender.clone();
        // Crea un nuovo thread per lo screen sharing
        let handle = Some(thread::spawn(move || {
            // Passiamo stdin e altri dati al thread
            start_screen_sharing(monitor, stop_flag, send);
        }));
        self.set_handle(handle.unwrap());
    }

    /*  pub fn clean_options(&mut self) {
        self.option.crop_area = None;
    }*/

    pub fn set_handle(&mut self, handle: JoinHandle<()>) {
        self.streaming_handle = Some(handle);
    }

    pub fn set_display(&mut self, monitor: Monitor) {
        let mut lock_mon = self.monitor_chosen.lock().unwrap();
        *lock_mon = monitor;
    }
/* 
    pub fn set_coordinates(&mut self, x: f64, y: f64, start_x: f64, start_y: f64) {
        self.option.crop_area = Some(Area {
            origin: Point {
                x: start_x,
                y: start_y,
            },
            size: Size {
                width: x,
                height: y,
            },
        });
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
    }*/

    pub fn get_available_displays(&self) -> Vec<Monitor> {
        return Monitor::all().unwrap();
    }

    pub fn stop_streaming(&mut self) {
        if self.stop_flag.load(Ordering::Relaxed) {
            return;
        }
        // Imposta il flag per fermare il thread
        self.stop_flag.store(true, Ordering::Relaxed);

        // Aspetta che il thread di streaming termini (se esiste)
        if let Some(handle) = self.streaming_handle.take() {
            handle
                .join()
                .expect("Errore nella terminazione del thread di streaming");
        }
    }
    
    pub fn take_screenshot(&mut self) -> Vec<u8> {

        take_screenshot(self.monitor_chosen.clone())
    }
    
    pub fn set_is_just_stopped(&mut self, value: bool) {
        self.is_just_stopped = value;
    }
    pub fn get_measures(&self) -> (u32, u32) {
        let lock_monitor = self.monitor_chosen.lock().unwrap();
        let x =  lock_monitor.width();
        let y = lock_monitor.height();
        println!("width {}, height {}", x, y);
        return (x,y);
    }
    /*
    pub fn get_measures(&self) -> (u32, u32) {
        match self.option.output_resolution {
            scap::capturer::Resolution::_480p => (640, 480), // 480p: 640x480
            scap::capturer::Resolution::_720p => (1280, 720), // 720p: 1280x720
            scap::capturer::Resolution::_1080p => (1920, 1080), // 1080p: 1920x1080
            scap::capturer::Resolution::_1440p => (1440, 900), // 1440p: 2560x1440
            scap::capturer::Resolution::_2160p => (3840, 2160), // 2160p: 3840x2160
            scap::capturer::Resolution::_4320p => (7680, 4320), // 4320p: 7680x4320
            scap::capturer::Resolution::Captured => (1920, 1080),
        }
    }*/
    pub fn get_trasmission_shortcut(&self)->String{
        println!("ciaooo {}", from_key_code_to_string(self.shortcut.get_manage_trasmition_shortcut()).to_string());
       from_key_code_to_string(self.shortcut.get_manage_trasmition_shortcut()).to_string()
    }
    pub fn get_blanking_screen(&self)->String{
        from_key_code_to_string(self.shortcut.get_blanking_screen_shortcut()).to_string()
    }
    pub fn get_terminate_screen(&self)->String{
        from_key_code_to_string(self.shortcut.get_terminate_session_shortcut()).to_string()
    }
    pub fn set_trasmission_shortcut(&mut self, shorcut:String){
        self.shortcut.set_manage_trasmition(shorcut.as_str())
    }
    pub fn set_blanking_screen(&mut self, shorcut:String){
        self.shortcut.set_blanking_screen(shorcut.as_str())
    }
    pub fn set_terminate_screen(&mut self,shorcut:String){
        self.shortcut.set_terminate_session(shorcut.as_str())
    }

}
