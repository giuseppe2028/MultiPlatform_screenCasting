use scap::{
    capturer::{Capturer, Options}, targets::Display, Target
};
use std::process::ChildStdin;
use std::sync::{Arc, Mutex};
use std::thread;
use iced::keyboard::KeyCode::M;
use crate::screenshare::screenshare::start_screen_sharing;

pub struct AppController {
    pub capturer: Arc<Mutex<Capturer>>,
    pub child: Option<Arc<Mutex<ChildStdin>>>,
    pub option: Options,
}

impl AppController {
    // Costruttore per AppController
    pub fn new(option: Options, child_stdin: Option<ChildStdin>) -> Self {
        let capturer = Capturer::new(option.clone());
        AppController {
            capturer:Arc::new(Mutex::new(capturer)),
            child: Some(Arc::new(Mutex::new(child_stdin.unwrap()))),
            option,
        }
    }

    // Funzione che avvia la condivisione dello schermo
    pub fn start_sharing(&mut self) {
        let capturer = Arc::clone(&self.capturer);
        let child_stdin = Arc::clone(self.child.as_ref().expect("SI E' ROTTO TUTTO"));

        // Crea un thread separato per eseguire `start_screen_sharing`
        thread::spawn(move || {
            // Esegui la funzione di cattura dello schermo in un thread separato
            start_screen_sharing(capturer, child_stdin);
        });
    }

    pub fn set_options(&mut self, options: Options) {
        self.option = options;
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

    pub fn set_child(&mut self, child: ChildStdin) {
        self.child = Some(Arc::new(Mutex::new(child)));
    }
}
