use scap::{
    capturer::{Capturer, Options}, targets::Display, Target
};
use std::process::ChildStdin;

use crate::screenshare::screenshare::start_screen_sharing;

pub struct AppController {
    pub capturer: Capturer,
    pub child: Option<ChildStdin>,
    pub option: Options,
}

impl AppController {
    // Costruttore per AppController
    pub fn new(option: Options, child_stdin: Option<ChildStdin>) -> Self {
        let capturer = Capturer::new(option.clone());
        AppController {
            capturer,
            child: child_stdin,
            option,
        }
    }

    // Funzione che avvia la condivisione dello schermo
    pub fn start_sharing(&mut self) {
        start_screen_sharing(&mut self.capturer, self.child.as_mut().expect("SI E' ROTTO TUTTO"));
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
        self.child = Some(child);
    }
}
