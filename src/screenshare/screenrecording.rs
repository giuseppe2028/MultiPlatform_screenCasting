use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

pub fn start_screen_recording(process_handle: Arc<Mutex<Option<Child>>>,options:Option<[(f64,f64);2]>) {
    #[cfg(target_os = "macos")]
    let ffmpeg_command = match options {
        None => {
            Command::new("ffmpeg")
                .args(&[
                    "-f", "avfoundation",  // Sorgente AVFoundation (macOS)
                    "-framerate", "30",    // Frame rate
                    "-i", "1",             // Indice del dispositivo (modifica se necessario)
                    "-video_size", "1920x1080", // Risoluzione
                    "-c:v", "libx264",     // Codec video
                    "-preset", "fast",     // Preset per velocità
                    "-crf", "23",          // Qualità del file (23 è un buon bilanciamento)
                    "output.mp4",          // File di output
                ])
                .spawn()
        }
        Some(option) => {
            Command::new("ffmpeg")
                .args(&[
                    "-f", "avfoundation",  // Sorgente AVFoundation (macOS)
                    "-framerate", "30",    // Frame rate
                    "-i", "1",             // Indice del dispositivo (modifica se necessario)
                    "-video_size", "1920x1080", // Risoluzione
                    "-vf",&format!("crop={}:{}:{}:{}", option[0].0, option[0].1, option[1].0, option[1].1),
                    "-c:v", "libx264",     // Codec video
                    "-preset", "fast",     // Preset per velocità
                    "-crf", "23",          // Qualità del file (23 è un buon bilanciamento)
                    "output.mp4",          // File di output
                ])
                .spawn()
        }
    };
    match ffmpeg_command {
        Ok(child) => {
            println!("FFmpeg avviato. Registrazione in corso...");
            *process_handle.lock().unwrap() = Some(child);
        }
        Err(e) => {
            eprintln!("Impossibile avviare FFmpeg: {}", e);
        }
    }
    #[cfg(target_os = "windows")]
    {
        let ffmpeg_command = match options {
            None => {
                Command::new("ffmpeg")
                    .args(&[
                        "-f", "gdigrab",
                        "-framerate", "30",
                        "-i", "desktop",
                        "-video_size", "1920x1080",
                        "-c:v", "libx264",
                        "-preset", "fast",
                        "-crf", "23",
                        "output.mp4",
                    ])
                    .output()
                    .expect("Failed to execute ffmpeg for Windows desktop capture")
            }
            Some(option) => {
                Command::new("ffmpeg")
                    .args(&[
                        "-f", "avfoundation",  // Sorgente AVFoundation (macOS)
                        "-framerate", "30",    // Frame rate
                        "-i", "1",             // Indice del dispositivo (modifica se necessario)
                        "-video_size", "1920x1080", // Risoluzione
                        "-vf", &format!("crop={}:{}:{}:{}", option[0].0, option[0].1, option[1].0, option[1].1),
                        "-c:v", "libx264",     // Codec video
                        "-preset", "fast",     // Preset per velocità
                        "-crf", "23",          // Qualità del file (23 è un buon bilanciamento)
                        "output.mp4",          // File di output
                    ])
                    .spawn()
            }
        };
        match ffmpeg_command {
            Ok(child) => {
                println!("FFmpeg avviato. Registrazione in corso...");
                *process_handle.lock().unwrap() = Some(child);
            }
            Err(e) => {
                eprintln!("Impossibile avviare FFmpeg: {}", e);
            }
        }
    }
}

pub fn stop_recording(process_handle: Arc<Mutex<Option<Child>>>) {
    let mut handle = process_handle.lock().unwrap();
    if let Some(child) = handle.as_mut() {
        match child.kill() {
            Ok(_) => println!("Registrazione interrotta con successo."),
            Err(e) => eprintln!("Errore durante l'interruzione di FFmpeg: {}", e),
        }
    } else {
        println!("Nessun processo di registrazione attivo.");
    }

    // Assicurati di attendere la terminazione completa del processo
    if let Some(mut child) = handle.take() {
        match child.wait() {
            Ok(status) => println!("Processo terminato con stato: {}", status),
            Err(e) => eprintln!("Errore durante l'attesa del processo: {}", e),
        }
    }
}