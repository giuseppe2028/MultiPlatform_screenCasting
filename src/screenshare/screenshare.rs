use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use xcap::Monitor;

#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::shared::windef::HBITMAP;
#[cfg(target_os = "windows")]
use winapi::um::wingdi::{
    CreateCompatibleDC, DeleteDC, GetBitmapBits, GetObjectA, SelectObject, BITMAP,
};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{CopyIcon, GetCursorInfo, GetIconInfo, CURSORINFO, ICONINFO};
use xcap::image::{DynamicImage, GenericImageView, RgbImage, RgbaImage};
use mouse_position::mouse_position::Mouse;

pub fn start_screen_sharing(
    monitor: Arc<Mutex<Monitor>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<RgbaImage>>,
) {
    while !stop_flag.load(Ordering::Relaxed) {
        let frame_result = {
            let mon_lock = monitor.lock().unwrap();
            mon_lock.capture_image()
        };

        match frame_result {
            Ok(frame) => {
                let (width, height) = (frame.width(), frame.height());
                let mut raw_data = frame.clone().into_raw();
                #[cfg(target_os = "windows")]
                {
                    if let Some((cursor_x, cursor_y, hbm_color)) = get_cursor_data() {
                        // Converti le coordinate globali del cursore in coordinate relative al monitor
                        let monitor_lock = monitor.lock().unwrap();
                        if let Some((relative_x, relative_y)) =
                            convert_cursor_coordinates(cursor_x, cursor_y, &*monitor_lock)
                        {
                            if hbm_color.is_null() {
                                // Sovrapponi manualmente un cursore a forma di "I"
                                overlay_text_cursor(
                                    &mut raw_data,
                                    width,
                                    height,
                                    relative_x,
                                    relative_y,
                                );
                            } else {
                                // Sovrapponi il cursore normale usando la bitmap
                                overlay_cursor_on_frame(
                                    &mut raw_data,
                                    width,
                                    height,
                                    relative_x,
                                    relative_y,
                                    hbm_color,
                                );
                            }
                        }
                    }
                }
                #[cfg(target_os = "macos")]
                {
                    if let Some((cursor_x, cursor_y)) = get_cursor_position() {
                        overlay_cursor_on_frame(&mut raw_data, width, height, cursor_x, cursor_y);
                    }
                }

                // Verifica che la lunghezza del buffer sia corretta
                if raw_data.len() != ((width * height * 4)).try_into().unwrap() {
                    eprintln!(
                        "Errore: Dimensioni del buffer non valide! Lunghezza attesa: {}",
                        width * height * 4
                    );
                    return;
                }

                // Ricrea il frame da raw_data
                if let Some(new_frame) = RgbaImage::from_raw(width, height, raw_data) {
                    // Invia il nuovo frame tramite il sender
                    if let Err(send_err) = sender.send(new_frame) {
                        eprintln!("Errore nell'invio dei dati del frame: {:?}", send_err);
                    }
                } else {
                    eprintln!("Errore: impossibile ricreare il frame da raw_data");
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
    //useful to draw the standard cursor
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
fn overlay_text_cursor(
    raw_data: &mut Vec<u8>,
    frame_width: u32,
    frame_height: u32,
    cursor_x: i32,
    cursor_y: i32,
) {
    //useful to draw the cursor when it is on a text input
    // Definisci le dimensioni del cursore a forma di "I"
    let cursor_height = 20; // Altezza del cursore "I" in pixel
                            //let cursor_width: i32 = 10;   // Larghezza del cursore "I" in pixel

    //let adjusted_cursor_x = cursor_x - (cursor_width / 2); // utile per disegnarlo un po' più in alto per renderlo uguale alla realtà
    let adjusted_cursor_x = cursor_x + 10; //utile per disegnarlo un po' più a destra per renderlo uguale alla realtà

    for y in 0..cursor_height {
        let frame_y = cursor_y + y;
        if frame_y >= 0 && frame_y < frame_height as i32 {
            let frame_index = ((frame_y as usize * frame_width as usize
                + adjusted_cursor_x as usize)
                * 4) as usize;
            if frame_index + 3 < raw_data.len() {
                // Colore del cursore a forma di "I" (es. nero)
                raw_data[frame_index] = 255; // B
                raw_data[frame_index + 1] = 255; // G
                raw_data[frame_index + 2] = 255; // R
                raw_data[frame_index + 3] = 255; // A (opaco)
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn get_cursor_data() -> Option<(i32, i32, HBITMAP)> {
    //useful to
    unsafe {
        let mut cursor_info = CURSORINFO {
            cbSize: std::mem::size_of::<CURSORINFO>() as u32,
            ..std::mem::zeroed()
        };

        if GetCursorInfo(&mut cursor_info) == 0 {
            return None;
        }

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

        let adjusted_cursor_x = cursor_x - icon_info.xHotspot as i32;
        let adjusted_cursor_y = cursor_y - icon_info.yHotspot as i32;

        // Verifica se l'HBITMAP è valido
        if icon_info.hbmColor.is_null() && icon_info.hbmMask.is_null() {
            //this is the case of the text cursor
            // Se entrambe le bitmap sono NULL, è probabile che si tratti di un cursore speciale
            // come il cursore a forma di "I". In questo caso, possiamo gestirlo separatamente.
            // Ritorna una rappresentazione personalizzata o un segnale per sovrapporre manualmente un cursore "I".
            return Some((adjusted_cursor_x, adjusted_cursor_y, ptr::null_mut()));
            // Usa `ptr::null_mut()` come segnale speciale
        }

        Some((adjusted_cursor_x, adjusted_cursor_y, icon_info.hbmColor))
    }
}

#[cfg(target_os = "windows")]
fn convert_cursor_coordinates(
    global_x: i32,
    global_y: i32,
    monitor: &Monitor,
) -> Option<(i32, i32)> {
    //useful to understand in which monitor drawing the cursor
    // Ottieni la posizione del monitor nell'area desktop
    let monitor_x = monitor.x();
    let monitor_y = monitor.y();

    // Ottieni le dimensioni del monitor
    let monitor_width = monitor.width() as i32;
    let monitor_height = monitor.height() as i32;

    // Calcola le coordinate relative al monitor selezionato
    let relative_x = global_x - monitor_x;
    let relative_y = global_y - monitor_y;

    // Controlla se le coordinate relative rientrano nel monitor selezionato
    if relative_x >= 0
        && relative_x < monitor_width
        && relative_y >= 0
        && relative_y < monitor_height
    {
        Some((relative_x, relative_y))
    } else {
        None // Il cursore non si trova su questo monitor
    }
}


#[cfg(target_os = "macos")]
fn get_cursor_position() -> Option<(f64, f64)> {
    // Creare un CGEventSource
   /* let event_source = CGEventSource::new(()).ok()?; // Gestisce eventuali errori
    let event = CGEvent::new(event_source).ok()?;  // Crea l'evento con la sorgente
    let location = event.location();
    Some((location.x, location.y))*/
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => Some((x as f64,y as f64)),
        Mouse::Error => {
            print!("Errore get cursor");
            None
        }
    }
}

// Funzione per sovrapporre il cursore al frame catturato
#[cfg(target_os = "macos")]
fn overlay_cursor_on_frame(
    raw_data: &mut Vec<u8>,
    frame_width: u32,
    frame_height: u32,
    cursor_x: f64,
    cursor_y: f64,
) {
    // Definisci un cursore semplice, ad esempio un piccolo rettangolo nero, o usa un'immagine predefinita
    let cursor_size = 0;
    let x = cursor_x as i32;
    let y = cursor_y as i32;

    for i in 0..cursor_size {
        for j in 0..cursor_size {
            let frame_x = x + i;
            let frame_y = y + j;

            if frame_x >= 0
                && frame_y >= 0
                && (frame_x as u32) < frame_width
                && (frame_y as u32) < frame_height
            {
                let frame_index =
                    ((frame_y as usize * frame_width as usize + frame_x as usize) * 4) as usize;

                if frame_index + 3 < raw_data.len() {
                    raw_data[frame_index] = 0; // B
                    raw_data[frame_index + 1] = 0; // G
                    raw_data[frame_index + 2] = 0; // R
                    raw_data[frame_index + 3] = 255; // A (opaco)
                }
            }
        }
    }
}
