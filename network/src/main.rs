use crate::network::bind_socket;

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

    pub async fn send_message(socket: &UdpSocket, addr: &str) -> Result<usize, io::Error> {
        let bytes_sent = socket.send_to(b"ciao", addr).await?;
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
    network::send_message(&socket1, "127.0.0.1:8080").await.expect("Failed to send message");

    // Ricevi un messaggio su socket2
    if let Ok((received_bytes, message, src)) = network::receive_message(&socket2).await {
        println!("Received message from {}: {}", src, message);
    }
}