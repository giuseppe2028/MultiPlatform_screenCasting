use crate::screenshare::screenshare::{start_screen_sharing, take_screenshot};
use crate::socket::socket::CasterSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use xcap::image::RgbaImage;
use xcap::Monitor;

pub struct AppController {
    pub monitor_chosen: Arc<std::sync::Mutex<Monitor>>,
    pub streaming_task: Option<tokio::task::JoinHandle<()>>, // Use Tokio's JoinHandle
    stop_flag: Arc<AtomicBool>,
    sender: Arc<tokio::sync::mpsc::Sender<RgbaImage>>, // Tokio mpsc channel for async communication
    pub is_just_stopped: bool,
    socket: Arc<Mutex<CasterSocket>>,
}

impl AppController {
    // Constructor for AppController
    pub fn new(
        monitor: Monitor,
        sender: tokio::sync::mpsc::Sender<RgbaImage>,
        socket: CasterSocket,
    ) -> Self {
        AppController {
            monitor_chosen: Arc::new(std::sync::Mutex::new(monitor)),
            streaming_task: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender),
            is_just_stopped: false,
            socket: Arc::new(Mutex::new(socket)),
        }
    }

    pub fn set_socket(&mut self, socket: CasterSocket) {
        self.socket = Arc::new(Mutex::new(socket));
    }

    // Function to start screen sharing using Tokio async task
    pub fn start_sharing(&mut self) {
        self.stop_flag.store(false, Ordering::Relaxed);

        let monitor = self.monitor_chosen.clone();
        let stop_flag = Arc::clone(&self.stop_flag);
        let sender = self.sender.clone();
        let socket = self.socket.clone();

        // Spawn a Tokio async task for screen sharing
        let task = tokio::spawn(async move {
            start_screen_sharing(monitor, stop_flag, sender, socket).await;
        });

        self.set_task(task);
    }

    // Function to listen for receivers using the socket
    pub fn listens_for_receivers(&mut self) {
        let sock_lock = self.socket.blocking_lock();
        let rt = Runtime::new().unwrap();
        rt.block_on(sock_lock.listen_for_registration());
    }

    pub fn set_task(&mut self, task: tokio::task::JoinHandle<()>) {
        self.streaming_task = Some(task);
    }

    pub fn set_display(&mut self, monitor: Monitor) {
        let mut lock_mon = self.monitor_chosen.lock().unwrap();
        *lock_mon = monitor;
    }

    pub fn get_available_displays(&self) -> Vec<Monitor> {
        return Monitor::all().unwrap();
    }

    // Stop streaming, async-safe
    pub fn stop_streaming(&mut self) {
        if self.stop_flag.load(Ordering::Relaxed) {
            return;
        }
        // Set the flag to stop streaming
        self.stop_flag.store(true, Ordering::Relaxed);
        self.socket.blocking_lock().destroy()

        /*async {                                           CI SERVE VERAMENTE A QUALCOSA ASPETTARE CHE IL TASK FINISCA?? TANTO FINISCE UGUALMENTE...
            // Await the task to ensure it finishes (if any)
            if let Some(task) = self.streaming_task.take() {
                task.await.expect("Error in stopping the streaming task");
            }
        };*/
    }

    // Take a screenshot asynchronously
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
