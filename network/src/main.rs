use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use image::{ImageBuffer, RgbaImage};
use scap::capturer::{Capturer, Options};
use scap::frame::Frame;
use crate::network::bind_socket;
pub fn take_screenshot() ->Option<Vec<u8>>{
    // Check if the platform is supported
    if !scap::is_supported() {
        println!("❌ Platform not supported");
        return None;
    }

    // Check if we have permission to capture screen
    if !scap::has_permission() {
        println!("❌ Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            println!("❌ Permission denied");
            return None;
        }
    }

    // Create Options
    let options = Options {
        fps: 30,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_1080p,
        ..Default::default()
    };

    // Create Recorder with options
    let mut recorder = Capturer::new(options);

    // Start Capture
    recorder.start_capture();

    // Capture a single frame
    let frame = recorder.get_next_frame().expect("Error capturing frame");

    match frame {
        Frame::BGRA(frame) => {
            let frame_vec = frame.clone().data;

            // Convert the frame data into an ImageBuffer for saving or displaying
            let width = frame.width as u32;
            let height = frame.height as u32;

            // Create an RGBA image buffer from the BGRA frame data
            let img: RgbaImage = ImageBuffer::from_raw(width, height, frame_vec)
                .expect("Failed to create image buffer");

            // Save the image to a PNG file
            let path = Path::new("output.png");
            let file = File::create(path).unwrap();
            let w = BufWriter::new(file);


            return Some(frame.data);
            println!("✅ Frame salvato come 'output.png'");

        }
        _ => {
            println!("Unsupported frame format");
            return None;
        }
    }
}
mod network {
    use tokio::io;
    use tokio::net::UdpSocket;

    pub async fn bind_socket(addr: &str) -> UdpSocket {
        let socket = UdpSocket::bind(addr).await.expect("couldn't bind to address");
        socket
    }

    pub async fn connect_socket(socket: &mut UdpSocket, addr: &str) -> Result<(), io::Error> {
        socket.connect(addr).await?;
        Ok(())
    }

    pub async fn send_message(socket: &UdpSocket, addr: &str,buff:Vec<u8>) -> Result<usize, io::Error> {
        let bytes_sent = socket.send_to(&*buff, addr).await?;
        Ok(bytes_sent)
    }

    pub async fn receive_message(socket: &UdpSocket) -> Result<(usize, String, std::net::SocketAddr), io::Error> {
        let mut buf = [0; 10]; // Buffer per ricevere i dati
        let (received, src) = socket.recv_from(&mut buf).await?;

        // Converti il buffer in una stringa
        let message = String::from_utf8_lossy(&buf[..received]).to_string();

        println!("Received {} bytes: {:?}", received, message);
        Ok((received, message, src))
    }
}

#[tokio::main]
async fn main() {

    // Creo due socket
    let socket1 = network::bind_socket("127.0.0.1:34254").await;
    let socket2 = network::bind_socket("127.0.0.1:8080").await;

    // Invia un messaggio da socket1 a socket2
    network::send_message(&socket1, "127.0.0.1:8080",take_screenshot().unwrap()).await.expect("Failed to send message");

    // Ricevi un messaggio su socket2
    if let Ok((received_bytes, message, src)) = network::receive_message(&socket2).await {
        println!("Received message from {}: {}", src, message);
    }
}