use scap::{
    capturer::{Capturer, Options}, targets::Display, Target
};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::{Child, ChildStdin, Stdio};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::{JoinHandle, Thread, ThreadId};
use iced::keyboard::KeyCode::M;
use scap::frame::Frame;
use crate::screenshare::screenshare::start_screen_sharing;
use crate::screenshare::screenshare::stop_screen_sharing;

pub struct AppController {
    pub capturer: Arc<Mutex<Capturer>>,
    pub option: Options,
    pub streaming_handle:Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    pub child: Option<Arc<Mutex<Child>>>,
    sender: Arc<Sender<Vec<u8>>>
}

impl AppController {
    // Costruttore per AppController
    pub fn new(option: Options, sender: Sender<Vec<u8>>) -> Self {
        let capturer = Capturer::new(option.clone());
        AppController {
            capturer: Arc::new(Mutex::new(capturer)),
            child:None,
            option,
            streaming_handle:None,
            stop_flag:Arc::new(AtomicBool::new(false)),
            sender: Arc::new(sender)
        }
    }



    // Funzione che avvia la condivisione dello schermo
    pub fn start_sharing(&mut self) {
        self.stop_flag.store(false,Ordering::Relaxed);
        let mut child = std::process::Command::new("ffplay")
            .args(&[
                "-f", "rawvideo",         // Input format
                "-pixel_format", "rgb24", // Pixel format (you may change this based on your frame data)
                "-video_size", "1440x900", // Resolution (adjust as necessary)
                "-framerate", "120",      // Frame rate (adjust as necessary)
                "-"
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .expect("Errore nell'avviare ffplay. Assicurati che ffplay sia installato e nel PATH.");
        let capturer = Arc::clone(&self.capturer);
        let stop_flag = Arc::clone(&self.stop_flag);
        self.child = Some(Arc::new(Mutex::new(child)));
        let ch1 = self.child.clone();
        let send = self.sender.clone();
        // Crea un nuovo thread per lo screen sharing
        let handle = Some(thread::spawn(move || {
            // Passiamo stdin e altri dati al thread
            start_screen_sharing(capturer,ch1.unwrap(), stop_flag, send);
        }));
        self.set_handle(handle.unwrap());
    }

    pub fn set_options(&mut self, options: Options) {
        self.option = options;
    }

    pub fn set_handle(&mut self, handle:JoinHandle<()>) {
        self.streaming_handle = Some(handle);
    }

    pub fn set_display(&mut self, display: Display) {
        self.option.target = Some(Target::Display(display));
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
            handle.join().expect("Errore nella terminazione del thread di streaming");
        }println!("Ciao! Il thread di streaso.");
        stop_screen_sharing(self.capturer.clone());
        self.child.clone().unwrap().lock().unwrap().kill().expect("Err");
        //kill(Pid::from_raw(self.id as i32), Signal::SIGKILL).expect("Errore nell'invio del segnale");
        println!("Ciao! Il thread di streaming è stato chiuso.");
    }

}
