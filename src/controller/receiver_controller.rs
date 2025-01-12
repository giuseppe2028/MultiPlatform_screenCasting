use crate::screenshare::screenshare::start_screen_receiving;
use crate::socket::socket::{ReceiverSocket, RegistrationError};
use rand::{thread_rng, Rng};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs, thread};
use tokio::runtime::Runtime;
use tokio::sync::{mpsc::Sender, Mutex};
use tokio::task;
use xcap::image::RgbaImage;

pub struct ReceiverController {
    pub streaming_handle: Option<task::JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<RgbaImage>>,
    socket: Arc<Mutex<ReceiverSocket>>,
    pub is_recording: Arc<AtomicBool>,
    counter: Arc<Mutex<usize>>,
}

impl ReceiverController {
    pub fn new(sender: Sender<RgbaImage>, socket: ReceiverSocket) -> Self {
        ReceiverController {
            streaming_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender),
            socket: Arc::new(Mutex::new(socket)),
            is_recording: Arc::new(AtomicBool::new(false)),
            counter: Arc::new(Mutex::new(0)),
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
                //println!("Ho inviato la richiesta di registrazione!");
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
        //println!("Ho inviato la richiesta di disconessione!");
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

    pub fn start_recording(&self, image: RgbaImage) {
        if self.is_recording.load(Ordering::Relaxed) {
            /*println!(
                "sono dntro start recording {}",
                self.is_recording.load(Ordering::Relaxed)
            );*/
            let counter = Arc::clone(&self.counter);
            //println!("starto la recording...");
            let _ = thread::spawn(move || {
                let mut counter_guard = counter.blocking_lock(); // Clona il contatore
                *counter_guard += 1;
                let folder_path = "./target/monitors";
                let file_path = format!("{}/monitors-{}.png", folder_path, *counter_guard);
                // Crea la directory se non esiste
                if let Err(e) = fs::create_dir_all(folder_path) {
                    eprintln!("Error creating directory {}: {}", folder_path, e);
                    return;
                }

                if let Err(e) = image.save(&file_path) {
                    eprintln!("Error saving image: {}", e);
                } else {
                    //println!("Image saved to {}", path);
                }
            });
            // a.join().expect("TODO: panic message");
        }
    }

    pub fn stop_recording(&self) {
        if self.is_recording.load(Ordering::Relaxed) {
            let actual_fps = "8"; // Sostituisci con il frame rate effettivo
            let input_pattern = "monitors-%d.png";
            let output_file = Self::generate_random_filename();
            let working_dir = "./target/monitors"; // Specifica la directory di lavoro corretta

            let status = Command::new("ffmpeg")
                .current_dir(working_dir) // Imposta la directory di lavoro
                .arg("-framerate")
                .arg(actual_fps)
                .arg("-i")
                .arg(input_pattern)
                .arg("-c:v")
                .arg("libx264")
                .arg("-pix_fmt")
                .arg("yuv420p")
                .arg(output_file)
                .status()
                .expect("Failed to execute ffmpeg");

            if status.success() {
                println!("Video created successfully!");
            } else {
                eprintln!("ffmpeg failed with exit code: {}", status);
            }
            Self::clear_images_with_command(working_dir, "monitors-");
        }
    }
    #[cfg(target_os = "windows")]
    pub fn clear_images_with_command(directory: &str, _: &str) {
        let dir_path = std::path::Path::new(directory);
    
        if !dir_path.exists() {
            eprintln!("Directory '{}' does not exist!", directory);
            return;
        }
    
        match std::fs::read_dir(dir_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().and_then(|ext| ext.to_str()) == Some("png") {
                            if let Err(e) = std::fs::remove_file(&path) {
                                eprintln!("Failed to delete file {:?}: {}", path, e);
                            } else {
                               // println!("Deleted file: {:?}", path);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read directory: {}", e);
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn clear_images_with_command(directory: &str, pattern: &str) {
        // Costruisce il comando per eliminare i file corrispondenti
        let command = format!("{}/{}*", directory, pattern);

        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("rm -f {}", command)) // Usa il comando `rm` con il pattern
            .status();

        match status {
            Ok(status) if status.success() => {
                //println!("Successfully deleted all files matching pattern: {}*", pattern);
            }
            Ok(status) => {
                eprintln!("Command failed with exit code: {}", status);
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub fn clear_images_with_command(directory: &str, pattern: &str) {
        // Costruisce il comando per eliminare i file corrispondenti
        let command = format!("{}/{}*", directory, pattern);

        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("rm -f {}", command)) // Usa il comando `rm` con il pattern
            .status();

        match status {
            Ok(status) if status.success() => {
                //println!("Successfully deleted all files matching pattern: {}*", pattern);
            }
            Ok(status) => {
                eprintln!("Command failed with exit code: {}", status);
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }
    

    fn generate_random_filename() -> String {
        let mut rng = thread_rng();
        let random_number: u64 = rng.gen_range(1_000_000_000..10_000_000_000); // 10 cifre
        format!("video-{}.mp4", random_number)
    }
}
