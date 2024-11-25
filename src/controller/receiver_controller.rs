use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::{Mutex, mpsc::Sender};
use tokio::task;
use xcap::image::RgbaImage;
use crate::screenshare::screenshare::start_screen_receiving;
use crate::socket::socket::ReceiverSocket;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ReceiverController {
    pub streaming_handle: Option<task::JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<RgbaImage>>,
    socket: Arc<Mutex<ReceiverSocket>>,
}

impl ReceiverController {
    pub fn new(sender: Sender<RgbaImage>, socket: ReceiverSocket) -> Self {
        ReceiverController {
            streaming_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender),
            socket: Arc::new(Mutex::new(socket)),
        }
    }

    pub async fn set_socket(&mut self, socket: ReceiverSocket) {
        self.socket = Arc::new(Mutex::new(socket));
    }

    pub fn start_receiving(&mut self) {
        let stop_flag = Arc::clone(&self.stop_flag);
        let socket = self.socket.clone();
        let send = self.sender.clone();

        let handle = tokio::spawn(async move {
            start_screen_receiving(stop_flag, send, socket).await;
        });
        self.set_handle(Some(handle));
    }

    pub fn register(&self) {
        let sock_lock = self.socket.blocking_lock();
        let rt = Runtime::new().unwrap();
        rt.block_on(sock_lock.register_with_caster());
        println!("Ho inviato la richiesta di registrazione!");
    }

    pub fn set_handle(&mut self, handle: Option<task::JoinHandle<()>>) {
        self.streaming_handle = handle;
    }

    pub async fn stop_streaming(&mut self) {
        if self.stop_flag.load(Ordering::Relaxed) {
            return;
        }
        // Imposta il flag per fermare il thread
        self.stop_flag.store(true, Ordering::Relaxed);

        // Attendi che il task di streaming termini (se esiste)
        if let Some(handle) = self.streaming_handle.take() {
            handle.await.expect("Errore nella terminazione del task di streaming");
        }
    }
}
