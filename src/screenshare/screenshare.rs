use std::io::Write;
use std::process::{ChildStdin, Command, Stdio};
use scap::capturer::{Area, Capturer, Options};
use scap::frame::Frame;

    pub fn visualize_screen_sharing(){
        //start ffplay
        let mut child = Command::new("ffplay")
            .args(&[
                "-f", "rawvideo",         // Formato non compresso
                "-pixel_format", "rgb24",  // Formato dei pixel: BGR con 0 per il canale alfa
                "-video_size", "1440x900", // Risoluzione del video (modifica secondo necessità)
                "-framerate", "120",       // Framerate (modifica secondo necessità)
                "-"                       // Leggi dallo stdin
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Errore nell'avviare ffplay. Assicurati che ffplay sia installato e nel PATH.");

        let mut out = child.stdin.as_mut().expect("Impossibile ottenere stdin di ffplay");
        //TODO complete
    }

    pub fn start_screen_sharing( cap: &mut Capturer, out: &mut ChildStdin){
        cap.start_capture();
        let mut start_time: u64 = 0;
        loop{
            let frame = cap.get_next_frame().expect("Error");

            match frame {
                Frame::YUVFrame(frame) => {
                    out.write_all(&frame.luminance_bytes).expect("Failed to write luminance data");
                    out.write_all(&frame.chrominance_bytes).expect("Failed to write chrominance data");
                    println!(
                        "Received BGR0 frame of width {} and height {}",
                        frame.width, frame.height
                    );
                }
                Frame::BGR0(frame) => {
                    println!(
                        "Received BGR0 frame of width {} and height {}",
                        frame.width, frame.height
                    );
                    out.write_all(&*frame.data).unwrap()
                }
                Frame::RGB(frame) => {
                    if start_time == 0 {
                        start_time = frame.display_time;
                    }
                    println!(
                        "Received BGR0 frame of width {} and height {}",
                        frame.width, frame.height
                    );
                    out.write_all(&*frame.data).unwrap()
                }
                Frame::RGBx(frame) => {
                    println!(
                        "Recieved RGBx frame of width {} and height {}",
                        frame.width, frame.height
                    );
                    out.write_all(&*frame.data).unwrap()
                }
                Frame::XBGR(frame) => {
                    println!(
                        "Recieved xRGB frame of width {} and height {}",
                        frame.width, frame.height
                    );
                    out.write_all(&*frame.data).unwrap()
                }
                Frame::BGRx(frame) => {
                    println!(
                        "Recieved BGRx frame of width {} and height {}",
                        frame.width, frame.height
                    );
                    out.write_all(&*frame.data).unwrap()
                }
                Frame::BGRA(frame) => {
                    if start_time == 0 {
                        start_time = frame.display_time;
                    }println!(
                        "Received BGR0 frame of width {} and height {}",
                        frame.width, frame.height
                    );

                    out.write_all(&*frame.data).unwrap()
                }
            }
        }
    }

    pub fn stop_screen_sharing( recorder: &mut Capturer){
        // Stop Capture
        recorder.stop_capture();
    }

    pub fn check_issupported()->bool{
        scap::is_supported()
    }

    pub fn check_haspermission()->bool{
        scap::has_permission()
    }

    pub fn set_options()->Options{
        //TODO we can set the options
        return Options {
            fps: 120,
            show_cursor: true,
            show_highlight: true,
            excluded_targets: None,
            output_type: scap::frame::FrameType::RGB,
            output_resolution: scap::capturer::Resolution::_1441p,

            ..Default::default()
        };
    }

    pub fn create_capture(options: Options)->Capturer{
        Capturer::new(options)
    }



