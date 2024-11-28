use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::io::{Write, Result};
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
                    "-pix_fmt", "uyvy422", // Forza l'uso del formato pixel supportato
                    "-c:v", "libx264",     // Codec video
                    "-preset", "fast",     // Preset per velocità
                    "-crf", "23",          // Qualità del file (23 è un buon bilanciamento)
                    "output.mp4",          // File di output
                ])
                .stdin(Stdio::piped())
                .spawn()

        }
        Some(option) => {
            Command::new("ffmpeg")
                .args(&[
                    "-f", "avfoundation",  // Sorgente AVFoundation (macOS)
                    "-framerate", "30",    // Frame rate
                    "-i", "1",             // Indice del dispositivo (modifica se necessario)
                    "-video_size", "1920x1080", // Risoluzione
                    "-pix_fmt", "uyvy422", // Forza l'uso del formato pixel supportato
                    "-c:v", "libx264",     // Codec video
                    "-preset", "fast",     // Preset per velocità
                    "-crf", "23",          // Qualità del file (23 è un buon bilanciamento)
                    "output.mp4",          // File di output
                ])
                .stdin(Stdio::piped())
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
        if let Some(stdin) = child.stdin.as_mut() {
            if let Err(e) = writeln!(stdin, "q") {
                eprintln!("Errore durante l'invio del comando a FFmpeg: {}", e);
            } else {
                println!("Comando 'q' inviato a FFmpeg. Terminazione in corso...");
            }
        } else {
            eprintln!("Impossibile accedere a stdin del processo FFmpeg.");
        }
    } else {
        println!("Nessun processo di registrazione attivo.");
    }
}