use serde::{Deserialize, Serialize};
use std::{net::IpAddr, sync::Arc};
use std::collections::HashMap;
use tokio::{net::UdpSocket, sync::{watch, RwLock}};
use xcap::image::RgbaImage;
use thiserror::Error;

const MTU: usize = 1500; // Dimensione massima del pacchetto
const UDP_HEADER_SIZE: usize = 8; // Dimensione dell'header UDP
const IP_HEADER_SIZE: usize = 20; // Dimensione dell'header IP
const MAX_PAYLOAD: usize = MTU - UDP_HEADER_SIZE - IP_HEADER_SIZE; // Spazio disponibile per il payload UDP

#[derive(Serialize, Deserialize)]
pub struct SerializableImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}
impl SerializableImage {
  
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
//    ip_addr: String,
    socket: Arc<Option<UdpSocket>>,
    receiver_sockets: Arc<RwLock<Vec<String>>>,
    termination_tx: watch::Sender<bool>, // Mittente del segnale di terminazione
    termination_rx: watch::Receiver<bool>, // Ricevitore del segnale di terminazione
    notification_tx: watch::Sender<usize>, // Canale per notifiche

}

impl CasterSocket {

    pub async fn new(ip_addr: &str, notification_tx: watch::Sender<usize>) -> Self {
        let socket = UdpSocket::bind(ip_addr).await.unwrap();
        let receiver_sockets = Arc::new(RwLock::new(vec![]));
        let socket_clone = Arc::new(Some(socket));

        let (termination_tx, termination_rx) = watch::channel(false); // Canale per terminazione


        let instance = CasterSocket {
            receiver_sockets,
            //ip_addr: ip_addr.to_string(), SERVE DAVVERO?? 
            socket: socket_clone,
            termination_tx,
            termination_rx,
            notification_tx,
        };

        // Avvia il task per ascoltare le registrazioni
        let instance_clone = instance.clone();
        let mut termination_rx = instance_clone.termination_rx.clone();

        tokio::spawn(async move {
            instance_clone
                .listen_for_registration_unregistration(&mut termination_rx)
                .await;
        });

        instance
    }

    pub async fn send_to_receivers(&self, frame: RgbaImage) {
        if let Some(socket) = self.socket.as_ref() {
            let serializable_image = SerializableImage {
                width: frame.width(),
                height: frame.height(),
                data: frame.into_raw(),
            };

            let serialized = bincode::serialize(&serializable_image).unwrap();
            let total_packets = (serialized.len() + MAX_PAYLOAD - 8 - 1) / (MAX_PAYLOAD - 8);

            // Usa una read-lock per accedere ai destinatari
            let receivers = self.receiver_sockets.read().await;

            for address in &*receivers {
                for i in 0..total_packets {
                    let start = i * (MAX_PAYLOAD - 8);
                    let end = ((i + 1) * (MAX_PAYLOAD - 8)).min(serialized.len());
                    let chunk = &serialized[start..end];

                    let mut packet = Vec::with_capacity(MAX_PAYLOAD);
                    packet.extend(&(i as u32).to_be_bytes()); // Numero del pacchetto
                    packet.extend(&(total_packets as u32).to_be_bytes()); // Numero totale di pacchetti
                    packet.extend(chunk); // Dati del pacchetto

                    if let Err(e) = socket.send_to(&packet, address).await {
                        eprintln!(
                            "Errore durante l'invio del pacchetto {} a {}: {}",
                            i, address, e
                        );
                    }
                }
            }
        } else {
            eprintln!("Socket non inizializzato o distrutto.");
        }
    }

    pub async fn listen_for_registration_unregistration(
        &self,
        termination_rx: &mut watch::Receiver<bool>,
    ) {
        let mut buf = vec![0; 1024];
        loop {
            tokio::select! {
                result = async {
                    if let Some(socket) = self.socket.as_ref() {
                        socket.recv_from(&mut buf).await
                    } else {
                        Err(std::io::Error::new(std::io::ErrorKind::Other, "Socket non disponibile"))
                    }
                } => {
                    match result {
                        Ok((len, src)) => {
                            if let Ok(message) = bincode::deserialize::<RegistrationMessage>(&buf[..len]) {
                                match message.action {
                                    Action::Register => {
                                       //println!("Registrato: {}:{}", message.ip, message.port);
                                        let mut receivers = self.receiver_sockets.write().await;
                                        receivers.push(format!("{}:{}", message.ip, message.port));
                                        let viewer_count = receivers.len();
                                        let _ = self.notification_tx.send(viewer_count);
                                    }
                                    Action::Disconnect => {
                                        println!("Disconnesso: {}:{}", message.ip, message.port);
                                        let mut receivers = self.receiver_sockets.write().await;
                                        receivers.retain(|addr| addr != &format!("{}:{}", message.ip, message.port));
                                        let viewer_count = receivers.len();
                                        println!("{}", viewer_count);
                                        let _ = self.notification_tx.send(viewer_count);
                                    }
                                }
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
                _ = termination_rx.changed() => {
                    if *termination_rx.borrow() {
                        println!("Ricevuto segnale di terminazione. Esco dal ciclo.");
                        break;
                    }
                }
            }
        }
    }


    pub fn destroy(&mut self) {
        let _ = self.termination_tx.send(true); // Segnala al task di terminare
        self.socket = Arc::new(None);
        //println!("Socket Caster distrutta.");
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Action {
    Register,
    Disconnect,
}

#[derive(Serialize, Deserialize)]
struct RegistrationMessage {
    ip: String,
    port: u16,
    action: Action,
}

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("IP address is not valid")]
    InvalidIp,
    #[error("Port parsing failed")]
    PortParsingError,
    #[error("Socket is not initialized")]
    SocketNotInitialized,
    #[error("Connection reset by the remote host")]
    ConnectionReset,
    #[error("Host unreachable")]
    NetworkUnreachable,
    #[error("Unknown error: {0}")]
    UnknownError(String),
}


#[derive(Clone, Debug)]
pub struct ReceiverSocket {
    ip_addr_caster: String,
    ip_addr: String,
    socket: Arc<Option<UdpSocket>>,
}

impl ReceiverSocket {
    pub async fn new(ip_addr_receiver: &str, ip_addr_caster: &str) -> Self {
        let socket = UdpSocket::bind(ip_addr_receiver).await.unwrap();
        ReceiverSocket {
            ip_addr_caster: ip_addr_caster.to_string(),
            ip_addr: ip_addr_receiver.to_string(),
            socket: Arc::new(Some(socket)),
        }
    }

    pub async fn receive_from(
        &self,
    ) -> Result<SerializableImage, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(socket) = self.socket.as_ref() {
            let mut buf = vec![0u8; MAX_PAYLOAD];
            let mut received_packets = HashMap::new();
            let mut total_packets = None;

            while total_packets.map_or(true, |total| received_packets.len() < total as usize) {
                let received_bytes = socket.recv(&mut buf).await?;
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
        } else {
            Err("Socket non disponibile".into())
        }
    }

    pub async fn register_with_caster(
        &self,
    ) -> Result<(), RegistrationError> {
        // Controlla se l'indirizzo IP del caster è valido
        let ip_parts: Vec<&str> = self.ip_addr_caster.split(':').collect();
        if ip_parts.len() != 2 {
            return Err(RegistrationError::InvalidIp);
        }
    
        // Valida l'IP
        let ip = ip_parts[0].parse::<IpAddr>();
        if ip.is_err() {
            return Err(RegistrationError::InvalidIp);
        }

        // Valida la porta
        let port = ip_parts[1].parse::<u16>();
        if port.is_err() {
            return Err(RegistrationError::PortParsingError);
        }

        let ip_parts_receiver: Vec<&str> = self.ip_addr.split(':').collect();
        let ip_receiver = ip_parts_receiver[0].parse::<IpAddr>().unwrap();
        let port_receiver = ip_parts_receiver[1].parse::<u16>().unwrap();
        println!("Receiver: {} {}", ip_receiver, port_receiver);
        
        // Crea il messaggio di registrazione
        let message = RegistrationMessage {
            ip: ip_receiver.to_string(),
            port: port_receiver,
            action: Action::Register,
        };
    
        let serialized = match bincode::serialize(&message) {
            Ok(data) => data,
            Err(_) => return Err(RegistrationError::UnknownError("Serialization failed".into())),
        };
    
        // Controlla se la socket è disponibile
        if let Some(socket) = self.socket.as_ref() {
            // Invia il messaggio di registrazione
            match socket.send_to(&serialized, &self.ip_addr_caster).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    match e.kind() {
                        tokio::io::ErrorKind::ConnectionReset => Err(RegistrationError::ConnectionReset),
                        tokio::io::ErrorKind::AddrNotAvailable => Err(RegistrationError::NetworkUnreachable),
                        _ => Err(RegistrationError::UnknownError(e.to_string())),
                    }
                }
            }
        } else {
            Err(RegistrationError::SocketNotInitialized)
        }
    }

    // Invia un messaggio di disconnessione al caster
    pub async fn unregister_with_caster(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = RegistrationMessage {
            ip: self.ip_addr.split(':').next().unwrap().to_string(),
            port: self.ip_addr.split(':').nth(1).unwrap().parse().unwrap(),
            action: Action::Disconnect,
        };

        let serialized = bincode::serialize(&message)?;

        if let Some(socket) = self.socket.as_ref() {
            socket.send_to(&serialized, &self.ip_addr_caster).await?;
            Ok(())
        } else {
            Err("Socket non inizializzata".into())
        }
    }

    pub fn destroy(&mut self) {
        self.socket = Arc::new(None);
        //println!("Socket Receiver distrutta.");
    }
}
