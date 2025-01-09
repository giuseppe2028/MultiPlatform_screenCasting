use crate::socket::socket::{CasterSocket, ReceiverSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time::timeout;
use xcap::image::RgbaImage;
use xcap::Monitor;

#[cfg(target_os = "macos")]
use mouse_position::mouse_position::Mouse;
#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "linux")]
use std::ptr;
#[cfg(target_os = "linux")]
use std::slice;
#[cfg(target_os = "windows")]
use winapi::shared::windef::HBITMAP;
#[cfg(target_os = "windows")]
use winapi::um::wingdi::{
    CreateCompatibleDC, DeleteDC, GetBitmapBits, GetObjectA, SelectObject, BITMAP,
};
#[cfg(target_os = "windows")]
use winapi::um::winuser::{CopyIcon, GetCursorInfo, GetIconInfo, CURSORINFO, ICONINFO};
#[cfg(target_os = "linux")]
use x11::xfixes::*;
#[cfg(target_os = "linux")]
use x11::xlib;
#[cfg(target_os = "linux")]
use x11::xlib::*;

pub async fn start_screen_sharing(
    monitor: Arc<std::sync::Mutex<Monitor>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<tokio::sync::mpsc::Sender<RgbaImage>>,
    socket: Arc<tokio::sync::Mutex<Option<CasterSocket>>>,
    blanking_flag: Arc<AtomicBool>,
) {
    while !stop_flag.load(Ordering::Relaxed) {
        // Cattura lo schermo in un task bloccante
        let frame_result = tokio::task::spawn_blocking({
            let monitor = monitor.clone();
            move || {
                let monitor_lock = monitor.lock().unwrap(); // Usa blocking_lock per operazioni sincrone
                monitor_lock.capture_image(None)
            }
        })
        .await; // Aspetta il risultato del task bloccante

        // Gestione dell'errore del task
        let frame = match frame_result {
            Ok(Ok(frame)) => frame, // Task completato con successo
            Ok(Err(e)) => {
                eprintln!("Error capturing screen: {:?}", e);
                continue;
            }
            Err(e) => {
                eprintln!("Task failed: {:?}", e);
                continue;
            }
        };

        let (width, height) = (frame.width(), frame.height());
        let mut raw_data = frame.into_raw();

        // Overlay del cursore per Windows
        #[cfg(target_os = "windows")]
        {
            if let Some((cursor_x, cursor_y, hbm_color)) = get_cursor_data() {
                let relative_coordinates = {
                    let monitor_lock = monitor.lock().unwrap();
                    convert_cursor_coordinates(cursor_x, cursor_y, &*monitor_lock)
                };

                if let Some((relative_x, relative_y)) = relative_coordinates {
                    if hbm_color.is_null() {
                        overlay_text_cursor(&mut raw_data, width, height, relative_x, relative_y);
                    } else {
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

        // Overlay del cursore per macOS
        #[cfg(target_os = "macos")]
        {
            if let Some((cursor_x, cursor_y)) = get_cursor_position() {
                overlay_cursor_on_frame(&mut raw_data, width, height, cursor_x, cursor_y);
            }
        }

        // Overlay del cursore per Linux
        #[cfg(target_os = "linux")]
        {
            if let Some(cursor_data) = get_cursor_image() {
                let monitor_lock = monitor.lock().unwrap();
                let cursor_coords =
                    convert_cursor_coordinates(cursor_data.3, cursor_data.4, &*monitor_lock, None);
                if let Some((adjusted_x, adjusted_y)) = cursor_coords {
                    let adjusted_cursor_data = (
                        cursor_data.0,
                        cursor_data.1,
                        cursor_data.2,
                        adjusted_x,
                        adjusted_y,
                    );
                    overlay_cursor_on_frame(&mut raw_data, width, height, &adjusted_cursor_data);
                }
            }
        }

        // Validazione della lunghezza del buffer
        if raw_data.len() != (width * height * 4) as usize {
            eprintln!(
                "Invalid buffer length: expected {}, got {}",
                width * height * 4,
                raw_data.len()
            );
            continue;
        }

        // Creazione del nuovo frame
        if let Some(new_frame) = RgbaImage::from_raw(width, height, raw_data) {
            // Invia il frame al canale
            if let Err(send_err) = sender.send(new_frame.clone()).await {
                eprintln!("Error sending frame data: {:?}", send_err);
            }

            // Invia il frame ai socket dei peer
            let sock_lock = socket.lock().await;
            if let Some(sock) = sock_lock.as_ref() {

                if blanking_flag.load(Ordering::Relaxed) {
                    //frame nero
                    //println!("Mando frame nero");
                    let black_frame_data = vec![0u8; (width * height * 4) as usize]; // RGBA: 4 byte per pixel
                    if let Some(black_frame) = RgbaImage::from_raw(width, height, black_frame_data)
                    {
                        sock.send_to_receivers(black_frame).await;
                    } else {
                        eprintln!("Error creating black frame");
                    }
                } else {
                        sock.send_to_receivers(new_frame).await;
                }
            } else {
                eprintln!("No CasterSocket available");
            }
        } else {
            eprintln!("Error recreating the frame from raw data");
        }
    }
    println!("Stopped sending frames");
}

pub async fn start_screen_receiving(
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<RgbaImage>>,
    socket: Arc<Mutex<ReceiverSocket>>,
) {
    println!("inizio a ricevere");
    while !stop_flag.load(Ordering::Relaxed) {
        // non possiamo metterla four dal while perchè si bugga nela chiusura
        let sock_lock = socket.lock().await;

        let start_time = tokio::time::Instant::now();

        //iniziare un timer qui
        // Timeout di 1 secondo per la ricezione
        match timeout(Duration::from_secs(100), sock_lock.receive_from()).await {
            Ok(Ok(serialized_image)) => {
                //stoppare il timer qui e stampare quanto tempo è passato
                if let Some(image) = RgbaImage::from_raw(
                    serialized_image.width(),
                    serialized_image.height(),
                    serialized_image.data().to_vec(),
                ) {
                    /*println!(
                        "Received a frame of size {}x{}",
                        image.width(),
                        image.height()
                    );*/
                    if let Err(send_err) = sender.send(image).await {
                        eprintln!("Error sending frame data: {:?}", send_err);
                    }
                } else {
                    eprintln!("Error creating RgbaImage from received data");
                }
                let elapsed_time = start_time.elapsed();
                println!("Ricezione completata in {} ms", elapsed_time.as_millis());
            }
            Ok(Err(e)) => {
                eprintln!("Error receiving frame {}", e);
            }
            Err(_) => {
                // Timeout scaduto, controlla lo stop_flag
                if stop_flag.load(Ordering::Relaxed) {
                    break;
                }
            }
        }
    }
    println!("Stopped receiving frames.");
}

pub async fn start_partial_sharing(
    monitor: Arc<std::sync::Mutex<Monitor>>,
    stop_flag: Arc<AtomicBool>,
    sender: Arc<Sender<RgbaImage>>,
    dimensions: [(f64, f64); 2],
    socket: Arc<tokio::sync::Mutex<Option<CasterSocket>>>,
    blanking_flag: Arc<AtomicBool>,
) {
    while !stop_flag.load(Ordering::Relaxed) {
        let frame_result = {
            let mon_lock = monitor.lock().unwrap();
            mon_lock.capture_image(Some([dimensions[0], dimensions[1]]))
        };

        match frame_result {
            Ok(frame) => {
                let (width, height) = (frame.width(), frame.height());
                let mut raw_data = frame.clone().into_raw();
                #[cfg(target_os = "windows")]
                {
                    if let Some((cursor_x, cursor_y, hbm_color)) = get_cursor_data() {
                        // Converti le coordinate globali del cursore in coordinate relative alla finestra selezionata
                        let monitor_lock = monitor.lock().unwrap();
                        if let Some((relative_x, relative_y)) = convert_cursor_coordinates_partial(
                            cursor_x,
                            cursor_y,
                            &*monitor_lock,
                            Some(dimensions),
                        ) {
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
                        } else {
                            println!("finita");
                        }
                    }
                }
                #[cfg(target_os = "macos")]
                {
                    if let Some((cursor_x, cursor_y)) = get_cursor_position() {
                        overlay_cursor_on_frame(&mut raw_data, width, height, cursor_x, cursor_y);
                    }
                }
                #[cfg(target_os = "linux")]
                {
                    if let Some(cursor_data) = get_cursor_image() {
                        let monitor_lock = monitor.lock().unwrap();
                        let cursor_coords = convert_cursor_coordinates(
                            cursor_data.3,
                            cursor_data.4,
                            &*monitor_lock,
                            Some(dimensions),
                        );
                        if let Some((adjusted_x, adjusted_y)) = cursor_coords {
                            let adjusted_cursor_data = (
                                cursor_data.0,
                                cursor_data.1,
                                cursor_data.2,
                                adjusted_x,
                                adjusted_y,
                            );
                            overlay_cursor_on_frame(
                                &mut raw_data,
                                width,
                                height,
                                &adjusted_cursor_data,
                            );
                        }
                    } else {
                        eprintln!("Impossibile ottenere il cursore.");
                    }
                }

                // Verifica che la lunghezza del buffer sia corretta
                let expected_len: usize = (width * height * 4).try_into().unwrap();
                if raw_data.len() != expected_len {
                    eprintln!(
                        "Errore: Dimensioni del buffer non valide! Lunghezza attesa: {}",
                        width * height * 4
                    );
                    return;
                }

                // Ricrea il frame da raw_data
                if let Some(new_frame) = RgbaImage::from_raw(width, height, raw_data) {
                    // Invia il nuovo frame tramite il sender
                    if let Err(send_err) = sender.send(new_frame.clone()).await {
                        eprintln!("Error sending frame data: {:?}", send_err);
                    }

                    let sock_lock = socket.lock().await;
                    if let Some(sock) = sock_lock.as_ref() {

                        if blanking_flag.load(Ordering::Relaxed) {
                            //frame nero
                            //println!("Mando frame nero");
                            let black_frame_data = vec![0u8; (width * height * 4) as usize]; // RGBA: 4 byte per pixel
                            if let Some(black_frame) =
                                RgbaImage::from_raw(width, height, black_frame_data)
                            {
                                sock.send_to_receivers(black_frame).await;
                            } else {
                                eprintln!("Error creating black frame");
                            }
                        } else {
                            sock.send_to_receivers(new_frame).await;
                        }
                    } else {
                        eprintln!("No CasterSocket available");
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

pub fn take_screenshot(monitor: Arc<std::sync::Mutex<Monitor>>) -> RgbaImage {
    let frame_result = {
        let mon_lock = monitor.lock().unwrap();
        mon_lock.capture_image(None)
    };

    match frame_result {
        Ok(frame) => {
            // Estrai i dati del buffer in formato raw
            //println!("FATTO SCREENSHOT");
            frame
        }
        Err(e) => {
            // Gestione dell'errore: registrare o stampare l'errore
            eprintln!("Errore durante la cattura dello schermo: {:?}", e);
            RgbaImage::new(1440, 900)
        }
    }
}

#[cfg(target_os = "windows")]
fn convert_cursor_coordinates_partial(
    global_x: i32,
    global_y: i32,
    monitor: &Monitor,
    dimensions: Option<[(f64, f64); 2]>, // Aggiungi le dimensioni opzionali
) -> Option<(i32, i32)> {
    // Ottieni la posizione del monitor nell'area desktop
    let monitor_x = monitor.x();
    let monitor_y = monitor.y();

    // Ottieni le dimensioni del monitor
    let monitor_width = monitor.width() as i32;
    let monitor_height = monitor.height() as i32;

    // Calcola le coordinate relative al monitor selezionato
    let mut relative_x = global_x - monitor_x;
    let mut relative_y = global_y - monitor_y;

    // Se stai condividendo una porzione dello schermo, sottrai gli offset
    if let Some(dim) = dimensions {
        let offset_x = dim[0].0 as i32;
        let offset_y = dim[0].1 as i32;
        relative_x -= offset_x;
        relative_y -= offset_y;
    }

    // Controlla se le coordinate relative rientrano nel monitor selezionato o nella porzione
    if relative_x >= 0
        && relative_x < monitor_width
        && relative_y >= 0
        && relative_y < monitor_height
    {
        Some((relative_x, relative_y))
    } else {
        None // Il cursore non si trova su questo monitor o porzione
    }
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
        Mouse::Position { x, y } => Some((x as f64, y as f64)),
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

#[cfg(target_os = "linux")]
fn get_cursor_position() -> Option<(i32, i32)> {
    unsafe {
        let display = xlib::XOpenDisplay(std::ptr::null());
        if display.is_null() {
            return None;
        }

        let root = xlib::XDefaultRootWindow(display);
        let mut root_x: i32 = 0;
        let mut root_y: i32 = 0;
        let mut win_x: i32 = 0;
        let mut win_y: i32 = 0;
        let mut mask: u32 = 0;
        let mut child: xlib::Window = 0;

        xlib::XQueryPointer(
            display,
            root,
            &mut child,
            &mut child,
            &mut root_x,
            &mut root_y,
            &mut win_x,
            &mut win_y,
            &mut mask,
        );

        xlib::XCloseDisplay(display);
        Some((root_x, root_y))
    }
}

// #[cfg(target_os = "linux")]
// fn overlay_cursor_on_frame(
//     raw_data: &mut Vec<u8>,
//     frame_width: u32,
//     frame_height: u32,
//     cursor_x: i32,
//     cursor_y: i32,
// ) {
//     // Simple cursor representation (white crosshair)
//     let cursor_size = 20;
//     let half_size = cursor_size / 2;

//     // Draw vertical line
//     for y in -half_size..half_size {
//         let frame_y = cursor_y + y;
//         if frame_y >= 0 && frame_y < frame_height as i32 {
//             let frame_x = cursor_x;
//             if frame_x >= 0 && frame_x < frame_width as i32 {
//                 let index = ((frame_y as u32 * frame_width + frame_x as u32) * 4) as usize;
//                 if index + 3 < raw_data.len() {
//                     raw_data[index] = 255; // B
//                     raw_data[index + 1] = 255; // G
//                     raw_data[index + 2] = 255; // R
//                     raw_data[index + 3] = 255; // A
//                 }
//             }
//         }
//     }

//     // Draw horizontal line
//     for x in -half_size..half_size {
//         let frame_x = cursor_x + x;
//         if frame_x >= 0 && frame_x < frame_width as i32 {
//             let frame_y = cursor_y;
//             if frame_y >= 0 && frame_y < frame_height as i32 {
//                 let index = ((frame_y as u32 * frame_width + frame_x as u32) * 4) as usize;
//                 if index + 3 < raw_data.len() {
//                     raw_data[index] = 255; // B
//                     raw_data[index + 1] = 255; // G
//                     raw_data[index + 2] = 255; // R
//                     raw_data[index + 3] = 255; // A
//                 }
//             }
//         }
//     }
// }
#[cfg(target_os = "linux")]
fn overlay_cursor_on_frame(
    raw_data: &mut Vec<u8>,
    frame_width: u32,
    frame_height: u32,
    cursor_data: &(Vec<u64>, u32, u32, i32, i32),
) {
    let (pixels, cursor_width, cursor_height, cursor_x, cursor_y) = cursor_data;

    for cy in 0..*cursor_height as i32 {
        for cx in 0..*cursor_width as i32 {
            let frame_x = cursor_x + cx;
            let frame_y = cursor_y + cy;

            if frame_x >= 0
                && frame_x < frame_width as i32
                && frame_y >= 0
                && frame_y < frame_height as i32
            {
                let frame_index = ((frame_y as u32 * frame_width + frame_x as u32) * 4) as usize;
                let cursor_index = (cy as u32 * *cursor_width + cx as u32) as usize;

                if frame_index + 3 < raw_data.len() && cursor_index < pixels.len() {
                    let pixel = pixels[cursor_index];
                    let a = ((pixel >> 24) & 0xFF) as f32 / 255.0;
                    let r = ((pixel >> 16) & 0xFF) as u8;
                    let g = ((pixel >> 8) & 0xFF) as u8;
                    let b = (pixel & 0xFF) as u8;

                    // Alpha blending
                    raw_data[frame_index] =
                        (raw_data[frame_index] as f32 * (1.0 - a) + b as f32 * a) as u8;
                    raw_data[frame_index + 1] =
                        (raw_data[frame_index + 1] as f32 * (1.0 - a) + g as f32 * a) as u8;
                    raw_data[frame_index + 2] =
                        (raw_data[frame_index + 2] as f32 * (1.0 - a) + r as f32 * a) as u8;
                    raw_data[frame_index + 3] = 255; // Set alpha to full opacity
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn get_cursor_image() -> Option<(Vec<u64>, u32, u32, i32, i32)> {
    unsafe {
        // Apri una connessione al display X11
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            eprintln!("Errore: impossibile aprire il display X11.");
            return None;
        }

        // Ottieni l'immagine del cursore
        let cursor_image = XFixesGetCursorImage(display);
        if cursor_image.is_null() {
            eprintln!("Errore: impossibile ottenere l'immagine del cursore.");
            XCloseDisplay(display);
            return None;
        }

        // Estrai i dati dell'immagine
        let width = (*cursor_image).width;
        let height = (*cursor_image).height;
        let x = (*cursor_image).x;
        let y = (*cursor_image).y;

        // Copia i pixel in un `Vec<u32>`
        let pixels_ptr = (*cursor_image).pixels;
        let pixels = slice::from_raw_parts(pixels_ptr, (width * height) as usize);
        let pixels_vec = pixels.to_vec();

        // Libera la memoria allocata
        XFree(cursor_image as *mut _);
        XCloseDisplay(display);

        // Restituisci l'immagine del cursore
        Some((pixels_vec, width as u32, height as u32, x as i32, y as i32))
    }
}

#[cfg(target_os = "linux")]
fn convert_cursor_coordinates(
    global_x: i32,
    global_y: i32,
    monitor: &Monitor,
    dimensions: Option<[(f64, f64); 2]>,
) -> Option<(i32, i32)> {
    let monitor_x = monitor.x();
    let monitor_y = monitor.y();
    let monitor_width = monitor.width() as f64;
    let monitor_height = monitor.height() as f64;

    // Coordinate relative al monitor
    let relative_x = global_x - monitor_x;
    let relative_y = global_y - monitor_y;

    if let Some(dims) = dimensions {
        let selected_x = dims[0].0;
        let selected_y = dims[0].1;
        let selected_width = dims[1].0 - dims[0].0;
        let selected_height = dims[1].1 - dims[0].1;

        // Calcola le proporzioni
        let width_ratio = monitor_width / selected_width;
        let height_ratio = monitor_height / selected_height;

        // Usa il rapporto più piccolo per mantenere l'aspetto
        let scale_factor = width_ratio.min(height_ratio);

        // Compensazione aggiuntiva per finestre piccole
        let size_compensation = if selected_width < monitor_width / 2.0 {
            let compensation_factor = (monitor_width / selected_width) * 0.15; // Aumentato il fattore di compensazione
            (compensation_factor, compensation_factor * 1.25) // Maggiore compensazione per l'asse Y
        } else {
            (1.0, 1.0)
        };

        // Calcola la posizione relativa considerando la scala e la compensazione
        let x_within_selection =
            (relative_x as f64 - selected_x) * scale_factor * size_compensation.0;
        let y_within_selection =
            (relative_y as f64 - selected_y) * scale_factor * size_compensation.1;

        // Calcola gli offset per centrare l'area scalata
        let scaled_width = selected_width * scale_factor;
        let scaled_height = selected_height * scale_factor;
        let x_offset = (monitor_width - scaled_width) / 2.0;
        let y_offset = (monitor_height - scaled_height) / 2.0;

        // Applica gli offset con compensazione aggiuntiva per l'asse Y
        let final_x = (x_within_selection + x_offset) as i32;
        let final_y = ((y_within_selection + y_offset) * 1.1) as i32; // Riduzione del 15% per spostare verso l'alto

        // Verifica che il cursore sia all'interno dell'area scalata
        if final_x >= 0
            && final_x < monitor_width as i32
            && final_y >= 0
            && final_y < monitor_height as i32
        {
            Some((final_x, final_y))
        } else {
            None
        }
    } else {
        // Per la cattura a schermo intero
        if relative_x >= 0
            && relative_x < monitor_width as i32
            && relative_y >= 0
            && relative_y < monitor_height as i32
        {
            Some((relative_x, relative_y))
        } else {
            None
        }
    }
}
