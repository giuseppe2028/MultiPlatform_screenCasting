use std::process::{Command, exit};
use futures::lock;
use xcap::Monitor;

use crate::screenshare::screenshare::{start_screen_sharing, take_screenshot};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use time::Duration;
use xcap::image::RgbaImage;
use crate::screenshare::screenrecording::start_screen_recording;

pub struct AppController {
    pub monitor_chosen: Arc<Mutex<Monitor>>,
    pub streaming_handle: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    stop_recording: Arc<AtomicBool>,
    sender: Arc<Sender<RgbaImage>>,
    pub is_just_stopped: bool,
    pub is_just_recorded:bool
}

impl AppController {
    // Costruttore per AppController
    pub fn new(monitor: Monitor, sender: Sender<RgbaImage>) -> Self {
        AppController {
            monitor_chosen: Arc::new(Mutex::new(monitor)),
            streaming_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            stop_recording: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender),
            is_just_stopped: false,
            is_just_recorded:false
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

    pub fn set_is_just_recorded(&mut self, value: bool) {
        self.is_just_recorded = value;
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
    pub fn start_recording(&mut self){
        let frame = 20;
        let start = Instant::now();
        let monitor = self.monitor_chosen.clone();
        let monitor = monitor.clone().lock().unwrap().clone();
        let stop_flag = Arc::clone(&self.stop_flag);
        let handle = Some(thread::spawn(move || {
            // Passiamo stdin e altri dati al thread
            start_screen_recording(monitor,stop_flag)
        }));

        self.set_handle(handle.unwrap());


        /*println!("time {:?}", start.elapsed());
        let actual_fps = 900 / start.elapsed().as_secs();
        println!("actual fps: {}", actual_fps);*/
    }

    pub fn stop_recording(&mut self){
        if self.stop_flag.load(Ordering::Relaxed) {
            return;
        }

        // Set the flag to stop the thread
        self.stop_flag.store(true, Ordering::Relaxed);

        // Wait for the streaming thread to finish (if it exists)
        if let Some(handle) = self.streaming_handle.take() {
            handle
                .join()
                .expect("Errore nella terminazione del thread di streaming");
        }

        // Execute ffmpeg command after recording has stopped
        let actual_fps = 2; // Replace with the actual FPS if needed
        let output_file = "output.mp4"; // The output file name
        let command = format!("ffmpeg -framerate {} -i target/monitors/recording-%d.png -c:v libx264 -pix_fmt yuv420p {}", actual_fps, output_file);

        let output = Command::new("bash")
            .arg("-c")
            .arg(command)
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!("Error executing ffmpeg: {}", String::from_utf8_lossy(&output.stderr));
                } else {
                    println!("ffmpeg command executed successfully: {}", String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(e) => {
                eprintln!("Failed to execute ffmpeg: {}", e);
                exit(1); // Exit if ffmpeg fails to start
            }
        }
    }
}
