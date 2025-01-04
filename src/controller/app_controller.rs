use crate::screenshare::screenshare::{start_partial_sharing, start_screen_sharing, take_screenshot,};
use crate::socket::socket::CasterSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use xcap::image::RgbaImage;
use xcap::Monitor;

#[derive(Debug, Clone)]
pub struct AppController {
    pub monitor_chosen: Arc<std::sync::Mutex<Monitor>>,
    stop_flag: Arc<AtomicBool>,
    blanking_flag: Arc<AtomicBool>,
    sender: Arc<tokio::sync::mpsc::Sender<RgbaImage>>, // Tokio mpsc channel for async communication
    pub is_just_stopped: bool,
    socket: Arc<Mutex<Option<CasterSocket>>>,
}

impl AppController {
    // Constructor for AppController
    pub fn new(
        monitor: Monitor,
        sender: tokio::sync::mpsc::Sender<RgbaImage>,
        socket: Option<CasterSocket>,
    ) -> Self {
        AppController {
            monitor_chosen: Arc::new(std::sync::Mutex::new(monitor)),
            stop_flag: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender),
            is_just_stopped: false,
            socket: Arc::new(Mutex::new(socket)),
            blanking_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_socket(&mut self, socket: CasterSocket) {
        self.socket = Arc::new(Mutex::new(Some(socket)));
    }

    // Function to start screen sharing using Tokio async task
    pub fn start_sharing(&mut self) {
        self.stop_flag.store(false, Ordering::Relaxed);

        let monitor = self.monitor_chosen.clone();
        let stop_flag = Arc::clone(&self.stop_flag);
        let sender = self.sender.clone();
        let socket = self.socket.clone();
        let blanking_flag = Arc::clone(&self.blanking_flag);

        // Spawn a Tokio async task for screen sharing
        let _ = tokio::spawn(async move {
            start_screen_sharing(monitor, stop_flag, sender, socket, blanking_flag).await;
        });

    }

    pub fn start_sharing_partial_sharing(&mut self, dimensions: [(f64, f64); 2]) {
        
        self.stop_flag.store(false, Ordering::Relaxed);
        let monitor = self.monitor_chosen.clone();
        let stop_flag = Arc::clone(&self.stop_flag);
        let send = self.sender.clone();
        let socket = self.socket.clone();
        let blanking_flag = Arc::clone(&self.blanking_flag);

        // Crea un nuovo thread per lo screen sharing
        let _ = tokio::spawn(async move {
            // Passiamo stdin e altri dati al thread
            start_partial_sharing(monitor, stop_flag, send, dimensions, socket, blanking_flag).await;
        });
    }

    pub fn set_display(&mut self, monitor: Monitor) {
        let mut lock_mon = self.monitor_chosen.lock().unwrap();
        *lock_mon = monitor;
    }

    // Stop streaming, async-safe
    pub async fn close_streaming(&mut self) {
        if self.stop_flag.load(Ordering::Relaxed) {
            return;
        }
        // Set the flag to stop streaming
        self.stop_flag.store(true, Ordering::Relaxed);

        // Distruggi la socket, se presente
        if let Some(socket) = self.socket.lock().await.as_mut() {
            println!("Socket distrutta");
            socket.destroy();
        }
    }

    pub fn stop_streaming(&mut self) {
        if self.stop_flag.load(Ordering::Relaxed) {
            return;
        }
        // Set the flag to stop streaming
        self.stop_flag.store(true, Ordering::Relaxed)
    }

    pub fn blanking_streaming(&mut self) {
        if self.blanking_flag.load(Ordering::Relaxed) {
            self.blanking_flag.store(false, Ordering::Relaxed)
        } else {
            self.blanking_flag.store(true, Ordering::Relaxed)
        }
    }

    pub fn take_screenshot(&mut self) -> RgbaImage {
        take_screenshot(self.monitor_chosen.clone())
    }
    pub fn set_is_just_stopped(&mut self, value: bool) {
        self.is_just_stopped = value;
    }

    pub fn get_measures(&self) -> (u32, u32) {
        let lock_monitor = self.monitor_chosen.lock().unwrap();
        let x = lock_monitor.width();
        let y = lock_monitor.height();
        //println!("width {}, height {}", x, y);
        return (x, y);
    }
}
