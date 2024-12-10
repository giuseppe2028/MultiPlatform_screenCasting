use iced::keyboard::Key;
use iced::keyboard::key::Named::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Backspace, Delete, End, Enter, Escape, F1, F10, F11, F12, F2, F3, F4, F5, F6, F7, F8, F9, Home, Insert, PageDown, PageUp, Space, Tab};
use serde_json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::error::Error;
use std::io::Write;
use std::str::FromStr;
/// La struttura Shortcut
#[derive(Debug)]
pub struct ShortcutController {
    manage_trasmition: Key,
    blanking_screen: Key,
    terminate_session: Key,
}

impl ShortcutController {
    pub fn new_from_file() -> Self {
        let path = "src/model/test_shortcuts.json";

        // Prova ad aprire il file
        let file = File::open(path);
        println!("result {:?}",file);
        let shortcuts: HashMap<String, String> = match file {
            Ok(f) => serde_json::from_reader(f).unwrap_or_else(|err| {
                eprintln!("Errore durante il parsing del JSON: {}. Uso valori di default.", err);
                HashMap::new()
            }),
            Err(e) => {
                eprintln!("File non trovato. Uso valori di default. errore: {}", e);
                HashMap::new() // Se il file non esiste, crea una mappa vuota
            }
        };

        // Recupera i valori dalla mappa o usa direttamente il valore di default
        let manage_trasmition = from_str_to_key(shortcuts.get("manage_trasmition").unwrap_or(&"No Shortcut selected".to_string()))
            .unwrap_or_else(|_| {
                eprintln!("Errore nel parsing di 'manage_trasmition'. Uso valore di default.");
                Key::Named(F1)
            });

        let blanking_screen = from_str_to_key(shortcuts.get("blanking_screen").unwrap_or(&"No Shortcut selected".to_string()))
            .unwrap_or_else(|_| {
                eprintln!("Errore nel parsing di 'blanking_screen'. Uso valore di default.");
                Key::Named(F1)
            });

        let terminate_session = from_str_to_key(shortcuts.get("terminate_session").unwrap_or(&"No Shortcut selected".to_string()))
            .unwrap_or_else(|_| {
                eprintln!("Errore nel parsing di 'terminate_session'. Uso valore di default.");
                Key::Named(F1)
            });

        ShortcutController {
            manage_trasmition,
            blanking_screen,
            terminate_session,
        }
    }
    pub fn set_manage_trasmition(&mut self,manage_trasmition:&str){
        self.manage_trasmition = from_str_to_key(manage_trasmition).unwrap();
        self.save_to_file().unwrap()
    }
    pub fn set_blanking_screen(&mut self,blanking_screen:&str){
        self.blanking_screen = from_str_to_key(blanking_screen).unwrap();
        self.save_to_file().unwrap()
    }
    pub fn set_terminate_session(&mut self,terminate_session:&str){
        self.terminate_session = from_str_to_key(terminate_session).unwrap();
        self.save_to_file().unwrap()
    }
    pub fn get_manage_trasmition_shortcut(&self)->Key{
        self.manage_trasmition.clone()
    }
    pub fn get_blanking_screen_shortcut(&self)->Key{
        self.blanking_screen.clone()
    }
    pub fn get_terminate_session_shortcut(&self)->Key{
        self.terminate_session.clone()
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let path = "src/model/test_shortcuts.json"; // Specifica il percorso del file
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

        // Crea una mappa chiave-valore con i valori correnti
        let shortcuts = HashMap::from([
            (
                "manage_trasmition".to_string(),
                from_key_to_string(self.manage_trasmition.clone()).to_string(),
            ),
            (
                "blanking_screen".to_string(),
                from_key_to_string(self.blanking_screen.clone()).to_string(),
            ),
            (
                "terminate_session".to_string(),
                from_key_to_string(self.terminate_session.clone()).to_string(),
            ),
        ]);

        // Scrivi la mappa come JSON nel file
        let json = serde_json::to_string_pretty(&shortcuts)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
/// Funzione per convertire una stringa in un Key
pub fn from_str_to_key(string: &str) -> Result<Key, String> {
    print!("{}",string.to_uppercase().as_str());
    match string.to_uppercase().as_str() {
        "A" => Ok(Key::Character("A".into())),
        "B" => Ok(Key::Character("B".into())),
        "C" => Ok(Key::Character("C".into())),
        "D" => Ok(Key::Character("D".into())),
        "E" => Ok(Key::Character("E".into())),
        "F" => Ok(Key::Character("F".into())),
        "G" => Ok(Key::Character("G".into())),
        "H" => Ok(Key::Character("H".into())),
        "I" => Ok(Key::Character("I".into())),
        "J" => Ok(Key::Character("J".into())),
        "K" => Ok(Key::Character("K".into())),
        "L" => Ok(Key::Character("L".into())),
        "M" => Ok(Key::Character("M".into())),
        "N" => Ok(Key::Character("N".into())),
        "O" => Ok(Key::Character("O".into())),
        "P" => Ok(Key::Character("P".into())),
        "Q" => Ok(Key::Character("Q".into())),
        "R" => Ok(Key::Character("R".into())),
        "S" => Ok(Key::Character("S".into())),
        "T" => Ok(Key::Character("T".into())),
        "U" => Ok(Key::Character("U".into())),
        "V" => Ok(Key::Character("V".into())),
        "W" => Ok(Key::Character("W".into())),
        "X" => Ok(Key::Character("X".into())),
        "Y" => Ok(Key::Character("Y".into())),
        "Z" => Ok(Key::Character("Z".into())),
        "1" => Ok(Key::Character("1".into())),
        "2" => Ok(Key::Character("2".into())),
        "3" => Ok(Key::Character("3".into())),
        "4" => Ok(Key::Character("4".into())),
        "5" => Ok(Key::Character("5".into())),
        "6" => Ok(Key::Character("6".into())),
        "7" => Ok(Key::Character("7".into())),
        "8" => Ok(Key::Character("8".into())),
        "9" => Ok(Key::Character("9".into())),
        "0" => Ok(Key::Character("0".into())),
        "ESCAPE" => Ok(Key::Named(Escape)),
        "SPACE" => Ok(Key::Named(Space)),
        "ENTER" => Ok(Key::Named(Enter)),
        "BACKSPACE" => Ok(Key::Named(Backspace)),
        "TAB" => Ok(Key::Named(Tab)),
        "INSERT" => Ok(Key::Named(Insert)),
        "DELETE" => Ok(Key::Named(Delete)),
        "HOME" => Ok(Key::Named(Home)),
        "END" => Ok(Key::Named(End)),
        "PAGEUP" => Ok(Key::Named(PageUp)),
        "PAGEDOWN" => Ok(Key::Named(PageDown)),
        "UP" | "ARROWUP" => Ok(Key::Named(ArrowUp)),
        "DOWN" | "ARROWDOWN" => Ok(Key::Named(ArrowDown)),
        "LEFT" | "ARROWLEFT" => Ok(Key::Named(ArrowLeft)),
        "RIGHT" | "ARROWRIGHT" => Ok(Key::Named(ArrowRight)),
        "F1" => Ok(Key::Named(F1)),
        "F2" => Ok(Key::Named(F2)),
        "F3" => Ok(Key::Named(F3)),
        "F4" => Ok(Key::Named(F4)),
        "F5" => Ok(Key::Named(F5)),
        "F6" => Ok(Key::Named(F6)),
        "F7" => Ok(Key::Named(F7)),
        "F8" => Ok(Key::Named(F8)),
        "F9" => Ok(Key::Named(F9)),
        "F10" => Ok(Key::Named(F10)),
        "F11" => Ok(Key::Named(F11)),
        "F12" => Ok(Key::Named(F12)),
        _ => Err(format!("Invalid Key: {}", string)),
    }
}

/// Funzione per convertire un Key in una stringa
pub fn from_key_to_string(key: Key) -> String {
    match key {
        Key::Named(Escape) => "Escape".into(),
        Key::Named(Space) => "Space".into(),
        Key::Named(Enter) => "Enter".into(),
        Key::Named(Backspace) => "Backspace".into(),
        Key::Named(Tab) => "Tab".into(),
        Key::Named(Insert) => "Insert".into(),
        Key::Named(Delete) => "Delete".into(),
        Key::Named(Home) => "Home".into(),
        Key::Named(End) => "End".into(),
        Key::Named(PageUp) => "PageUp".into(),
        Key::Named(PageDown) => "PageDown".into(),
        Key::Named(ArrowUp) => "ArrowUp".into(),
        Key::Named(ArrowDown) => "ArrowDown".into(),
        Key::Named(ArrowLeft) => "ArrowLeft".into(),
        Key::Named(ArrowRight) => "ArrowRight".into(),
        Key::Named(F1) => "F1".into(),
        Key::Named(F2) => "F2".into(),
        Key::Named(F3) => "F3".into(),
        Key::Named(F4) => "F4".into(),
        Key::Named(F5) => "F5".into(),
        Key::Named(F6) => "F6".into(),
        Key::Named(F7) => "F7".into(),
        Key::Named(F8) => "F8".into(),
        Key::Named(F9) => "F9".into(),
        Key::Named(F10) => "F10".into(),
        Key::Named(F11) => "F11".into(),
        Key::Named(F12) => "F12".into(),
        Key::Character(letter) => letter.to_uppercase().into(),

        _ => "Unknown".into(),
    }
}