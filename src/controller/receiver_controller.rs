use std::sync::{Arc, Mutex};
use crate::screenshare::screenshare::start_screen_receiving;
use crate::socket::socket::ReceiverSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

pub struct ReceiverController {
    pub streaming_handle: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    pub is_just_stopped: bool,
    socket: Arc<Mutex<ReceiverSocket>>,
}

impl ReceiverController {

    pub fn new(socket: ReceiverSocket) -> Self {
        ReceiverController {
            streaming_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            is_just_stopped: false,
            socket: Arc::new(Mutex::new(socket)),
        }
    }

    pub fn set_socket(&mut self, socket: ReceiverSocket) {
        self.socket = Arc::new(Mutex::new(socket));
    }

    pub fn start_receiving(&mut self) {
        let stop_flag = Arc::clone(&self.stop_flag);
        let socket = self.socket.clone();

        let handle = Some(thread::spawn(move || {
            // Passiamo stdin e altri dati al thread
            start_screen_receiving(stop_flag, socket);
        }));
        self.set_handle(handle.unwrap());
    }

    pub fn register(&mut self) {
        let sock_lock = self.socket.lock().unwrap();
        sock_lock.register_with_caster();
        println!("Ho inviato la richiesta di registrazioene!");
    }

    pub fn set_handle(&mut self, handle: JoinHandle<()>) {
        self.streaming_handle = Some(handle);
    }

    pub fn stop_streaming(&mut self) {
        if self.stop_flag.load(Ordering::Relaxed) {
            return;
        }
        // Imposta il flag per fermare il thread
        self.stop_flag.store(true, Ordering::Relaxed);

        //in teoria dovrei fare in modo qui di dire al Caster di non mandarmi più pacchetti??? oppure lo capisce da solooo?? boh

        // Aspetta che il thread di streaming termini (se esiste)
        if let Some(handle) = self.streaming_handle.take() {
            handle
                .join()
                .expect("Errore nella terminazione del thread di streaming");
        }
    }
}
