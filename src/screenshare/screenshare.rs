use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use xcap::Monitor;

#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::shared::windef::HBITMAP;
#[cfg(target_os = "windows")]
use winapi::um::wingdi::{CreateCompatibleDC, DeleteDC, GetBitmapBits, GetObjectA, SelectObject, BITMAP};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{CopyIcon, GetCursorInfo, GetIconInfo, CURSORINFO, ICONINFO};

pub fn start_screen_sharing(
    monitor: Arc<Mutex<Monitor>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<Vec<u8>>>,
) {
    while !stop_flag.load(Ordering::Relaxed) {
        let frame_result = {
            let mon_lock = monitor.lock().unwrap();
            mon_lock.capture_image()
        };

        match frame_result {
            Ok(frame) => {
                let (width, height) = (frame.width(), frame.height());
                let mut raw_data = frame.into_raw();

                #[cfg(target_os = "windows")]
                {
                    if let Some((cursor_x, cursor_y, hbm_color)) = get_cursor_data() {
                        overlay_cursor_on_frame(
                            &mut raw_data,
                            width,
                            height,
                            cursor_x,
                            cursor_y,
                            hbm_color,
                        );
                    }
                }

                if let Err(send_err) = sender.send(raw_data) {
                    eprintln!("Errore nell'invio dei dati del frame: {:?}", send_err);
                }
            }
            Err(e) => {
                eprintln!("Errore durante la cattura dello schermo: {:?}", e);
            }
        }
    }
}

/*pub fn stop_screen_sharing(capturer: Arc<Mutex<Option<Capturer>>>) {
    // Acquire the lock and stop capture if `capturer` is available
    let mut capturer_lock = capturer.lock().unwrap();
    if let Some(ref mut cap) = *capturer_lock {
        cap.stop_capture();
    }
}*/

/*
    FUNZIONI UTILI?
pub fn check_issupported() -> bool {
    scap::is_supported()
}

pub fn check_haspermission() -> bool {
    scap::has_permission()
}

pub fn set_options() -> Options {
    //TODO we can set the options
    return Options {
        fps: 120,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::RGB,
        output_resolution: scap::capturer::Resolution::_1440p,

        ..Default::default()
    };
}

pub fn create_capture(options: Options) -> Capturer {
    Capturer::new(options)
}
*/
pub fn take_screenshot(monitor: Arc<Mutex<Monitor>>) -> Vec<u8> {
    let frame_result = {
        let mon_lock = monitor.lock().unwrap();
        mon_lock.capture_image()
    };

    match frame_result {
        Ok(frame) => {
            // Estrai i dati del buffer in formato raw
            let raw_data = frame.into_raw(); // Questo metodo estrae i dati del buffer

            return raw_data;
        }
        Err(e) => {
            // Gestione dell'errore: registrare o stampare l'errore
            eprintln!("Errore durante la cattura dello schermo: {:?}", e);
        }
    }
    return vec![];
}

#[cfg(target_os = "windows")]
fn overlay_cursor_on_frame(
    raw_data: &mut Vec<u8>,
    frame_width: u32,
    frame_height: u32,
    cursor_x: i32,
    cursor_y: i32,
    hbm_color: HBITMAP,
) {
    unsafe {
        // Creare un HDC compatibile
        let hdc = CreateCompatibleDC(ptr::null_mut());
        if hdc.is_null() {
            return;
        }

        // Selezionare l'HBITMAP nel contesto di dispositivo (DC)
        let old_obj = SelectObject(hdc, hbm_color as *mut _);
        if old_obj.is_null() {
            DeleteDC(hdc);
            return;
        }

        // Ottieni le informazioni sul bitmap del cursore
        let mut bitmap = BITMAP {
            ..std::mem::zeroed()
        };
        if GetObjectA(
            hbm_color as *mut _,
            std::mem::size_of::<BITMAP>() as i32,
            &mut bitmap as *mut _ as *mut _,
        ) == 0
        {
            DeleteDC(hdc);
            return;
        }

        // Leggi i dati dei pixel del cursore
        let bitmap_size = (bitmap.bmWidth * bitmap.bmHeight * 4) as usize;
        let mut cursor_pixels = vec![0u8; bitmap_size];
        if GetBitmapBits(
            hbm_color,
            bitmap_size as i32,
            cursor_pixels.as_mut_ptr() as *mut _,
        ) == 0
        {
            DeleteDC(hdc);
            return;
        }

        // Sovrapponi i pixel del cursore al buffer del frame
        for y in 0..bitmap.bmHeight {
            for x in 0..bitmap.bmWidth {
                let cursor_index = ((y * bitmap.bmWidth + x) * 4) as usize;
                let frame_x = cursor_x + x;
                let frame_y = cursor_y + y;

                // Controlla se la posizione è all'interno dei limiti del frame
                if frame_x >= 0
                    && frame_y >= 0
                    && (frame_x as usize) < frame_width as usize
                    && (frame_y as usize) < frame_height as usize
                {
                    let frame_index =
                        ((frame_y as usize * frame_width as usize + frame_x as usize) * 4) as usize;

                    // Leggi i valori RGBA del cursore
                    let b = cursor_pixels[cursor_index];
                    let g = cursor_pixels[cursor_index + 1];
                    let r = cursor_pixels[cursor_index + 2];
                    let a = cursor_pixels[cursor_index + 3];

                    // Sovrapponi solo se il pixel del cursore non è completamente trasparente
                    if a > 0 {
                        // Applica un'operazione di alfa blending per sovrapporre il cursore
                        let alpha = a as f32 / 255.0;
                        raw_data[frame_index] =
                            (raw_data[frame_index] as f32 * (1.0 - alpha) + r as f32 * alpha) as u8;
                        raw_data[frame_index + 1] =
                            (raw_data[frame_index + 1] as f32 * (1.0 - alpha) + g as f32 * alpha)
                                as u8;
                        raw_data[frame_index + 2] =
                            (raw_data[frame_index + 2] as f32 * (1.0 - alpha) + b as f32 * alpha)
                                as u8;
                    }
                }
            }
        }

        // Pulisci le risorse GDI
        SelectObject(hdc, old_obj);
        DeleteDC(hdc);
    }
}


#[cfg(target_os = "windows")]
fn get_cursor_data() -> Option<(i32, i32, HBITMAP)> {
    unsafe {
        // Struttura per ottenere le informazioni sul cursore
        let mut cursor_info = CURSORINFO {
            cbSize: std::mem::size_of::<CURSORINFO>() as u32,
            ..std::mem::zeroed()
        };

        // Ottieni le informazioni sul cursore
        if GetCursorInfo(&mut cursor_info) == 0 {
            return None;
        }

        // Ottieni l'icona del cursore
        let hicon = CopyIcon(cursor_info.hCursor);
        if hicon.is_null() {
            return None;
        }

        let mut icon_info = ICONINFO {
            ..std::mem::zeroed()
        };

        if GetIconInfo(hicon, &mut icon_info) == 0 {
            return None;
        }

        // Ottieni la posizione del cursore
        let cursor_x = cursor_info.ptScreenPos.x;
        let cursor_y = cursor_info.ptScreenPos.y;

        // Ritorna la posizione e l'immagine del cursore (HBITMAP)
        Some((cursor_x, cursor_y, icon_info.hbmColor))
    }
}
