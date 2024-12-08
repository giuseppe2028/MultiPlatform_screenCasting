use crate::screenshare::screenshare::start_screen_receiving;
use crate::socket::socket::{ReceiverSocket, RegistrationError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::{mpsc::Sender, Mutex};
use tokio::task;
use xcap::image::RgbaImage;

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

    pub fn start_receiving(&mut self) {
        self.stop_flag.store(false, Ordering::Relaxed);

        let stop_flag = Arc::clone(&self.stop_flag);
        let socket = self.socket.clone();
        let send = self.sender.clone();

        let handle = tokio::spawn(async move {
            start_screen_receiving(stop_flag, send, socket).await;
        });
        self.set_handle(Some(handle));
    }

    pub fn register(&self) -> Result<String, String> {
        let mut sock_lock = self.socket.blocking_lock();
        let rt = Runtime::new().unwrap();
        match rt.block_on(sock_lock.register_with_caster()) {
            Ok(_) => {
                println!("Ho inviato la richiesta di registrazione!");
                Ok("Registrazione completata con successo!".to_string())
            }
            Err(e) => {
                sock_lock.destroy();
                let user_message = match e {
                    RegistrationError::InvalidIp => "L'indirizzo IP inserito non è valido.",
                    RegistrationError::PortParsingError => "La porta specificata non è valida.",
                    RegistrationError::SocketNotInitialized => "La socket non è stata inizializzata correttamente.",
                    RegistrationError::ConnectionReset => "Connessione interrotta dal caster.",
                    RegistrationError::NetworkUnreachable => "La rete non è raggiungibile. Controlla la tua connessione.",
                    RegistrationError::UnknownError(err) => &format!("{}", err),
                };
                println!("Errore durante la registrazione: {}", user_message);
                Err(user_message.to_string())
            }
        }
    }

    pub fn unregister(&self) {
        let sock_lock = self.socket.blocking_lock();
        let rt = Runtime::new().unwrap();
        let _ = rt.block_on(sock_lock.unregister_with_caster());
        println!("Ho inviato la richiesta di disconessione!");
    }

    pub fn set_handle(&mut self, handle: Option<task::JoinHandle<()>>) {
        self.streaming_handle = handle;
    }

    pub fn close_streaming(&mut self) {
        if self.stop_flag.load(Ordering::Relaxed) {
            return;
        }
        // Imposta il flag per fermare il thread
        self.stop_flag.store(true, Ordering::Relaxed);
        self.socket.blocking_lock().destroy();

        /*async {
            // Attendi che il task di streaming termini (se esiste)
            if let Some(handle) = self.streaming_handle.take() {
                handle
                    .await
                    .expect("Errore nella terminazione del task di streaming");
            }
        };*/
    }
}
