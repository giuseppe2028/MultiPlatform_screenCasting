use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::{net::UdpSocket, sync::RwLock};
use std::{sync::Arc};
use xcap::image::RgbaImage;

const MAX_UDP_PAYLOAD: usize = 65507;

#[derive(Serialize, Deserialize)]
pub struct SerializableImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl SerializableImage {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self { width, height, data }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

#[derive(Clone, Debug)]
pub struct CasterSocket {
    ip_addr: String,
    socket: Arc<UdpSocket>,
    receiver_sockets: Arc<RwLock<Vec<String>>>,
}

impl CasterSocket {
    pub async fn new(ip_addr: &str) -> Self {
        let socket = UdpSocket::bind(ip_addr).await.unwrap();
        CasterSocket {
            receiver_sockets: Arc::new(RwLock::new(vec![])),
            ip_addr: ip_addr.to_string(),
            socket: Arc::new(socket),
        }
    }

    pub async fn send_to_receivers(&self, frame: RgbaImage) {
        let serializable_image = SerializableImage {
            width: frame.width(),
            height: frame.height(),
            data: frame.into_raw(),
        };

        let serialized = bincode::serialize(&serializable_image).unwrap();
        let total_packets = (serialized.len() + MAX_UDP_PAYLOAD - 8 - 1) / (MAX_UDP_PAYLOAD - 8);

        // Usa una read-lock per accedere ai destinatari
        let receivers = self.receiver_sockets.read().await;

        for address in &*receivers {
            for i in 0..total_packets {
                let start = i * (MAX_UDP_PAYLOAD - 8);
                let end = ((i + 1) * (MAX_UDP_PAYLOAD - 8)).min(serialized.len());
                let chunk = &serialized[start..end];

                let mut packet = Vec::with_capacity(MAX_UDP_PAYLOAD);
                packet.extend(&(i as u32).to_be_bytes()); // Numero del pacchetto
                packet.extend(&(total_packets as u32).to_be_bytes()); // Numero totale di pacchetti
                packet.extend(chunk); // Dati del pacchetto

                if let Err(e) = self.socket.send_to(&packet, address).await {
                    eprintln!("Errore durante l'invio del pacchetto {} a {}: {}", i, address, e);
                }
            }
        }
    }

    pub async fn listen_for_registration(&self) {
        let mut buf = vec![0; 1024];
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            println!("aspettando registrazioni..");
            match self.socket.recv_from(&mut buf).await {
                Ok((len, src)) => {
                    if let Ok(message) = bincode::deserialize::<RegistrationMessage>(&buf[..len]) {
                        println!("Registrato: {}:{}", message.ip, message.port);
                        let mut receivers = self.receiver_sockets.write().await;
                        receivers.push(format!("{}:{}", message.ip, message.port));
                        break; //esco appena uno si registra
                    } else {
                        eprintln!("Ricevuto messaggio non valido da {}", src);
                    }
                }
                Err(e) => {
                    eprintln!("Errore durante la ricezione: {}", e);
                    break;
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RegistrationMessage {
    ip: String,
    port: u16,
}

#[derive(Clone, Debug)]
pub struct ReceiverSocket {
    ip_addr_caster: String,
    ip_addr: String,
    socket: Arc<UdpSocket>,
}

impl ReceiverSocket {
    pub async fn new(ip_addr_receiver: &str, ip_addr_caster: &str) -> Self {
        let socket = UdpSocket::bind(ip_addr_receiver).await.unwrap();
        ReceiverSocket {
            ip_addr_caster: ip_addr_caster.to_string(),
            ip_addr: ip_addr_receiver.to_string(),
            socket: Arc::new(socket),
        }
    }

    pub async fn receive_from(&self) -> Result<SerializableImage, Box<dyn std::error::Error + Send + Sync>> {
        let mut buf = vec![0u8; MAX_UDP_PAYLOAD];
        let mut received_packets = HashMap::new();
        let mut total_packets = None;

        while total_packets.map_or(true, |total| received_packets.len() < total as usize) {
            let received_bytes = self.socket.recv(&mut buf).await?;
            let packet_num = u32::from_be_bytes(buf[0..4].try_into()?);
            let total = u32::from_be_bytes(buf[4..8].try_into()?);

            if total_packets.is_none() {
                total_packets = Some(total);
            }

            received_packets.insert(packet_num, buf[8..received_bytes].to_vec());
        }

        let mut compressed_data = Vec::new();
        for i in 0..total_packets.unwrap() {
            compressed_data.extend(received_packets.remove(&i).unwrap());
        }

        let deserialized_image: SerializableImage = bincode::deserialize(&compressed_data)?;

        Ok(deserialized_image)
    }

    pub async fn register_with_caster(&self) {
        let message = RegistrationMessage {
            ip: self.ip_addr.split(':').next().unwrap().to_string(),
            port: self.ip_addr.split(':').nth(1).unwrap().parse().unwrap(),
        };

        let serialized = bincode::serialize(&message).unwrap();
        self.socket.send_to(&serialized, &self.ip_addr_caster).await.unwrap();
    }
}
