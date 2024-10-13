use std::process::ChildStdin;
use scap::capturer::{Capturer, Options};
use super::*; // Importa i moduli dal livello superiore (necessario per usare ChildStdin e Capturer)

    pub struct AppController {
        pub capturer: Capturer,
        pub child: ChildStdin,
        pub option:Options
    }

    impl AppController {
        // Costruttore per AppController
        pub fn new(option: Options,child_stdin: ChildStdin) -> Self {
            let capturer = Capturer::new(option.clone());
            AppController {
                capturer,
                child:child_stdin,
                option
            }
        }

        // Funzione che avvia la condivisione dello schermo
        pub fn start_sharing(&mut self) {

        }

        pub fn set_options(&mut self, options:Options){
            self.option=options;
        }
    }

