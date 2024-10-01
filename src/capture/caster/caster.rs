use tokio::net::UdpSocket;
use scrap::{Capturer, Display};
use std::io::Error;

pub async fn start_udp_server() -> Result<(), Error> {
    let local_address = "0.0.0.0:8080"; // Bind to this address for broadcasting
    let socket = UdpSocket::bind(local_address).await?;
    
    println!("UDP Server ready, waiting for clients...");
    
    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;

    // Buffer to store client address once we receive it
    let mut buf = vec![0; 1024];
    let mut client_addr = None;

    // Wait for a message from the client to capture its address
    let (n, addr) = socket.recv_from(&mut buf).await?;
    println!("Client connected from: {}", addr);
    client_addr = Some(addr);

    loop {
        if let Some(addr) = client_addr {
            // Capture a frame
            let frame = capturer.frame()?;
            
            // Send the frame to the client
            socket.send_to(&frame, addr).await?;
        }

        // Add some delay for smoother capturing (60 FPS)
        tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
    }
}
