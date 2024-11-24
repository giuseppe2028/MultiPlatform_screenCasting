use crate::screenshare::screenshare::{
    start_screen_receiving, start_screen_sharing, take_screenshot,
};
use crate::socket::socket::CasterSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use xcap::image::RgbaImage;
use xcap::Monitor;

pub struct AppController {
    pub monitor_chosen: Arc<Mutex<Monitor>>,
    pub streaming_handle: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<RgbaImage>>,
    pub is_just_stopped: bool,
    socket: Arc<Mutex<CasterSocket>>,
}

impl AppController {
    // Costruttore per AppController
    pub fn new(monitor: Monitor, sender: Sender<RgbaImage>, socket: CasterSocket) -> Self {
        AppController {
            monitor_chosen: Arc::new(Mutex::new(monitor)),
            streaming_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender),
            is_just_stopped: false,
            socket: Arc::new(Mutex::new(socket)),
        }
    }

    pub fn set_socket(&mut self, socket: CasterSocket) {
        self.socket = Arc::new(Mutex::new(socket));
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
        let socket = self.socket.clone();
        // Crea un nuovo thread per lo screen sharing
        let handle = Some(thread::spawn(move || {
            // Passiamo stdin e altri dati al thread
            start_screen_sharing(monitor, stop_flag, send, socket);
        }));
        self.set_handle(handle.unwrap());
    }

    pub fn listens_for_receivers(&mut self) {
        // Borrow the Arc, not the Arc itself
        let mut sock_lock = self.socket.lock().unwrap();
        sock_lock.listen_for_registration();
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
        let x = lock_monitor.width();
        let y = lock_monitor.height();
        println!("width {}, height {}", x, y);
        return (x, y);
    }
}
