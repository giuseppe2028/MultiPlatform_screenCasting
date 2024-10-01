mod screenshare {
    use std::io;
    use std::io::ErrorKind::WouldBlock;
    use std::io::{Write, Result};
    use std::process::{Child, Command, Stdio};
    use scrap::{Capturer, Display};

    pub fn create_display() -> Result<Display> {
        Display::primary().map_err(|e| {
            eprintln!("Failed to create display: {:?}", e);
            e
        })
    }

    pub fn start_capture(display: Display) -> Capturer {
        Capturer::new(display).expect("Failed to start capture.") // Spostiamo solo il valore
    }

    pub fn setup_ffplay(w:usize,h:usize) -> Child {
        Command::new("ffplay")
            .args(&[
                "-f", "rawvideo",
                "-pixel_format", "bgr0",
                "-video_size", &format!("{}x{}", w, h),
                "-framerate", "60",
                "-"
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("This example requires ffplay.")
    }

    pub fn recording(mut capturer: Capturer, w:usize,h:usize) -> Result<()> {
        let mut out = setup_ffplay(w,h).stdin.take().expect("Failed to get stdin");

        loop {
            match capturer.frame() {
                Ok(frame) => {
                    let stride = frame.len() / h;
                    let rowlen = 4 * w; // 4 bytes per pixel (RGBA)

                    for row in frame.chunks(stride) {
                        let row = &row[..rowlen]; // Trim padding
                        out.write_all(row).map_err(|e| {
                            eprintln!("Failed to write to ffplay stdin: {:?}", e);
                            e
                        })?;
                    }
                }
                Err(ref e) if e.kind() == WouldBlock => {
                    // Wait for the frame.
                }
                Err(e) => {
                    eprintln!("Capture error: {:?}", e);
                    break;
                }
            }
        }

        // Chiudi stdin dopo aver terminato la scrittura
        drop(out);
        Ok(())
    }
}

fn main() {
    match screenshare::create_display() {
        Ok(display) => {

            let (w,h) = (display.width(),display.height());
            print!("display: {} {}",w,h);
            let capturer = screenshare::start_capture(display); // Passa come riferimento
            if let Err(e) = screenshare::recording(capturer, w,h) { // Passa come riferimento
                eprintln!("Error during recording: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Error initializing display: {:?}", e);
        }
    }
}