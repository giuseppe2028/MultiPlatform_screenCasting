use serde_json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::error::Error;
use std::io::Write;
use iced::keyboard::KeyCode;
use std::str::FromStr;

/// La struttura Shortcut
#[derive(Debug)]
pub struct Shortcut {
    manage_trasmition: KeyCode,
    blanking_screen: KeyCode,
    terminate_session: KeyCode,
}

impl Shortcut {
    pub fn new_from_file() -> Result<Self, Box<dyn Error>> {
        let path = "src/model/test_shortcuts.json";
        // Apri il file
        let file = File::open(path)?;
        // Leggi il contenuto del file JSON in una mappa chiave-valore
        let shortcuts: HashMap<String, String> = serde_json::from_reader(file)?;

        // Converte i valori della mappa in KeyCode
        let manage_trasmition = from_str_to_key_code(&shortcuts["manage_trasmition"])?;
        let blanking_screen = from_str_to_key_code(&shortcuts["blanking_screen"])?;
        let terminate_session = from_str_to_key_code(&shortcuts["terminate_session"])?;

        Ok(Shortcut {
            manage_trasmition,
            blanking_screen,
            terminate_session,
        })
    }
    pub fn set_manage_trasmition(&mut self,manage_trasmition:&str){
       self.manage_trasmition = from_str_to_key_code(manage_trasmition).unwrap();
        self.save_to_file().unwrap()
    }
    pub fn set_blanking_screen(&mut self,blanking_screen:&str){
        self.blanking_screen = from_str_to_key_code(blanking_screen).unwrap();
        self.save_to_file().unwrap()
    }
    pub fn set_terminate_session(&mut self,terminate_session:&str){
        self.terminate_session = from_str_to_key_code(terminate_session).unwrap();
        self.save_to_file().unwrap()
    }
    pub fn get_manage_trasmition_shortcut(&self)->KeyCode{
        self.manage_trasmition
    }
    pub fn get_blanking_screen_shortcut(&self)->KeyCode{
        self.blanking_screen
    }
    pub fn get_terminate_session_shortcut(&self)->KeyCode{
        self.terminate_session
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let path = "src/model/test_shortcuts.json"; // Specifica il percorso del file
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

        // Crea una mappa chiave-valore con i valori correnti
        let shortcuts = HashMap::from([
            (
                "manage_trasmition".to_string(),
                from_key_code_to_string(self.manage_trasmition).to_string(),
            ),
            (
                "blanking_screen".to_string(),
                from_key_code_to_string(self.blanking_screen).to_string(),
            ),
            (
                "terminate_session".to_string(),
                from_key_code_to_string(self.terminate_session).to_string(),
            ),
        ]);

        // Scrivi la mappa come JSON nel file
        let json = serde_json::to_string_pretty(&shortcuts)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

/// Funzione per convertire una stringa in un KeyCode
pub fn from_str_to_key_code(string: &str) -> Result<KeyCode, String> {
    match string {
        "A" => Ok(KeyCode::A),
        "B" => Ok(KeyCode::B),
        "C" => Ok(KeyCode::C),
        "D" => Ok(KeyCode::D),
        "E" => Ok(KeyCode::E),
        "F" => Ok(KeyCode::F),
        "G" => Ok(KeyCode::G),
        "H" => Ok(KeyCode::H),
        "I" => Ok(KeyCode::I),
        "J" => Ok(KeyCode::J),
        "K" => Ok(KeyCode::K),
        "L" => Ok(KeyCode::L),
        "M" => Ok(KeyCode::M),
        "N" => Ok(KeyCode::N),
        "O" => Ok(KeyCode::O),
        "P" => Ok(KeyCode::P),
        "Q" => Ok(KeyCode::Q),
        "R" => Ok(KeyCode::R),
        "S" => Ok(KeyCode::S),
        "T" => Ok(KeyCode::T),
        "U" => Ok(KeyCode::U),
        "V" => Ok(KeyCode::V),
        "W" => Ok(KeyCode::W),
        "X" => Ok(KeyCode::X),
        "Y" => Ok(KeyCode::Y),
        "Z" => Ok(KeyCode::Z),
        "Key1" | "1" => Ok(KeyCode::Key1),
        "Key2" | "2" => Ok(KeyCode::Key2),
        "Key3" | "3" => Ok(KeyCode::Key3),
        "Key4" | "4" => Ok(KeyCode::Key4),
        "Key5" | "5" => Ok(KeyCode::Key5),
        "Key6" | "6" => Ok(KeyCode::Key6),
        "Key7" | "7" => Ok(KeyCode::Key7),
        "Key8" | "8" => Ok(KeyCode::Key8),
        "Key9" | "9" => Ok(KeyCode::Key9),
        "Key0" | "0" => Ok(KeyCode::Key0),
        "Escape" => Ok(KeyCode::Escape),
        "Space" => Ok(KeyCode::Space),
        "Enter" => Ok(KeyCode::Enter),
        "Backspace" => Ok(KeyCode::Backspace),
        "Tab" => Ok(KeyCode::Tab),
        "Insert" => Ok(KeyCode::Insert),
        "Delete" => Ok(KeyCode::Delete),
        "Home" => Ok(KeyCode::Home),
        "End" => Ok(KeyCode::End),
        "PageUp" => Ok(KeyCode::PageUp),
        "PageDown" => Ok(KeyCode::PageDown),
        "ArrowUp" | "Up" => Ok(KeyCode::Up),
        "ArrowDown" | "Down" => Ok(KeyCode::Down),
        "ArrowLeft" | "Left" => Ok(KeyCode::Left),
        "ArrowRight" | "Right" => Ok(KeyCode::Right),
        "F1" => Ok(KeyCode::F1),
        "F2" => Ok(KeyCode::F2),
        "F3" => Ok(KeyCode::F3),
        "F4" => Ok(KeyCode::F4),
        "F5" => Ok(KeyCode::F5),
        "F6" => Ok(KeyCode::F6),
        "F7" => Ok(KeyCode::F7),
        "F8" => Ok(KeyCode::F8),
        "F9" => Ok(KeyCode::F9),
        "F10" => Ok(KeyCode::F10),
        "F11" => Ok(KeyCode::F11),
        "F12" => Ok(KeyCode::F12),
        // Aggiungi altre chiavi secondo necessità
        _ => Err(format!("Invalid KeyCode: {}", string)),
    }
}

pub fn from_key_code_to_string(keyCode:KeyCode)->&'static str{
    match keyCode {
        KeyCode::Key1 => "1",
        KeyCode::Key2 => "2",
        KeyCode::Key3 => "3",
        KeyCode::Key4 => "4",
        KeyCode::Key5 => "5",
        KeyCode::Key6 => "6",
        KeyCode::Key7 => "7",
        KeyCode::Key8 => "8",
        KeyCode::Key9 => "9",
        KeyCode::Key0 => "10",
        KeyCode::A => "A",
        KeyCode::B => "B",
        KeyCode::C => "C",
        KeyCode::D => "D",
        KeyCode::E => "E",
        KeyCode::F => "F",
        KeyCode::G => "G",
        KeyCode::H => "H",
        KeyCode::I => "I",
        KeyCode::J => "J",
        KeyCode::K => "K",
        KeyCode::L => "L",
        KeyCode::M => "M",
        KeyCode::N => "N",
        KeyCode::O => "O",
        KeyCode::P => "P",
        KeyCode::Q => "Q",
        KeyCode::R => "R",
        KeyCode::S => "S",
        KeyCode::T => "T",
        KeyCode::U => "U",
        KeyCode::V => "V",
        KeyCode::W => "W",
        KeyCode::X => "X",
        KeyCode::Y => "Y",
        KeyCode::Z => "Z",
        _ => ""
    }
}
