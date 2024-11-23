use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use xcap::image::RgbaImage;
use xcap::Monitor;

pub fn start_screen_recording(
    monitor: Monitor,
    stop_flag: Arc<AtomicBool>
) {
    let mut i = 0;
    let frame_rate_ms = 1000 / 30; // 30 fps, quindi 33 ms per fotogramma
    let start = Instant::now();

    while !stop_flag.load(Ordering::Relaxed) {
        i += 1;
        let time = Instant::now();
        let image = monitor.capture_image().unwrap();
        save_image_async(image, format!("target/monitors/recording-{}.png", i));

        // Calcola il tempo da aspettare per raggiungere il frame rate
        let elapsed_time = start.elapsed().as_millis();
        let target_time = i * frame_rate_ms as u128;

        // Se elapsed_time supera target_time, significa che il fotogramma è stato acquisito troppo velocemente
        // e non è necessario fare nessuna pausa
        let sleep_time = if target_time > elapsed_time {
            target_time - elapsed_time
        } else {
            0 // Non attendere se il fotogramma è stato acquisito troppo velocemente
        };

        // Se il tempo di attesa è positivo, dorme per quel tempo
        if sleep_time > 0 {
            thread::sleep(Duration::from_millis(sleep_time as u64));
        }

        println!(
            "Frame {} - sleep_time: {:?} current_step_time: {:?}",
            i,
            sleep_time,
            time.elapsed()
        );

        // Interrompe la registrazione dopo aver acquisito 900 fotogrammi
        if i >= 900 {
            break;
        }
    }
}

fn save_image_async(image: RgbaImage, filename: String) {
    thread::spawn(move || {
        image.save(filename).unwrap();
    });
}