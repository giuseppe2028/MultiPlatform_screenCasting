use bincode::Encode;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::{net::UdpSocket, thread, time::Duration};
use xcap::image::RgbaImage;

const MAX_UDP_PAYLOAD: usize = 65507; // Imposta un limite inferiore al massimo teorico per sicurezza

#[derive(Serialize, Deserialize, Encode)] // Deriviamo il trait Serialize per rendere l'immagine serializzabile
pub struct SerializableImage {
    width: u32,
    height: u32,
    data: Vec<u8>, // Memorizziamo i dati dei pixel come un array di byte
}

impl SerializableImage {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data,
        }
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

pub struct CasterSocket {
    ip_addr: String,
    socket: UdpSocket,
    receiver_sockets: Vec<String>,
}

impl CasterSocket {
    pub fn new(ip_addr: &str) -> Self {
        let socket = UdpSocket::bind(ip_addr).unwrap();
        socket.set_nonblocking(true).unwrap(); //non farla bloccare nelle receive
        CasterSocket {
            receiver_sockets: vec![],       // Lista inizialmente vuota
            ip_addr: String::from(ip_addr), // Salva l'indirizzo IP
            socket: socket,                 // Inizializza il socket
        }
    }

    pub fn send_to_receivers(&self, frame: RgbaImage) {
        let serializable_image = SerializableImage {
            width: frame.width(),
            height: frame.height(),
            data: frame.into_raw(),
        };
    
        let serialized = serde_cbor::to_vec(&serializable_image).unwrap();
    
        // Comprimi i dati
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&serialized).unwrap();
        let compressed_data = encoder.finish().unwrap();
    
        // Calcola il numero totale di pacchetti
        let total_packets = (compressed_data.len() + MAX_UDP_PAYLOAD - 1) / MAX_UDP_PAYLOAD;
    
        for address in &self.receiver_sockets {
            for i in 0..total_packets {
                let start = i * (MAX_UDP_PAYLOAD - 8);  // Dedurre 8 byte per l'header
                let end = ((i + 1) * (MAX_UDP_PAYLOAD - 8)).min(compressed_data.len());
                let chunk = &compressed_data[start..end];
    
                // Prepara l'header: Numero di pacchetto (4 byte) + totale pacchetti (4 byte)
                let mut packet = Vec::new();
                packet.extend(&(i as u32).to_be_bytes()); // Numero di pacchetto
                packet.extend(&(total_packets as u32).to_be_bytes()); // Totale pacchetti
                packet.extend(chunk); // Aggiungi i dati del pacchetto
    
                // Verifica che la dimensione del pacchetto non superi il limite
                assert!(packet.len() <= MAX_UDP_PAYLOAD, "Pacchetto troppo grande!");
    
                match self.socket.send_to(&packet, address) {
                    Ok(n) => println!(
                        "Inviato pacchetto {} di {} ({} byte)",
                        i + 1,
                        total_packets,
                        n
                    ),
                    Err(e) => eprintln!("Errore nell'invio del pacchetto {}: {}", i + 1, e),
                }
            }
        }
    }

    pub fn listen_for_registration(&mut self) {
        let mut buf = [0; 1024];
        println!("Sto aspettando che qualcuno si registri!");

        if self.receiver_sockets.is_empty() {
            // Se non ci sono receiver registrati, rimani in loop finché non arriva almeno un messaggio valido
            loop {
                thread::sleep(Duration::from_millis(1000));
                match self.socket.recv_from(&mut buf) {
                    Ok((len, _src)) => {
                        let message: RegistrationMessage =
                            serde_cbor::from_slice(&buf[..len]).unwrap();
                        println!("Registrato: {}:{}", message.ip, message.port);
                        self.receiver_sockets
                            .push(format!("{}:{}", message.ip, message.port));
                        break; // Esci dal loop dopo aver registrato il primo receiver
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Nessun dato disponibile, continua a provare
                        println!("aspetto..");
                        continue;
                    }
                    Err(e) => {
                        eprintln!("Errore durante la ricezione: {}", e);
                        break; // Esci dal loop in caso di errore grave
                    }
                }
            }
        } else {
            // Se ci sono già receiver registrati, prova a ricevere una sola volta
            match self.socket.recv_from(&mut buf) {
                Ok((len, _src)) => {
                    let message: RegistrationMessage = serde_cbor::from_slice(&buf[..len]).unwrap();
                    println!("Registrato: {}:{}", message.ip, message.port);
                    self.receiver_sockets
                        .push(format!("{}:{}", message.ip, message.port));
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Nessun messaggio ricevuto: ritorna dalla funzione
                    println!("Nessun nuovo receiver da registrare.");
                }
                Err(e) => {
                    eprintln!("Errore durante la ricezione: {}", e);
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

pub struct ReceiverSocket {
    ip_addr_caster: String,
    ip_addr: String,
    socket: UdpSocket,
}

impl ReceiverSocket {
    pub fn new(ip_addr_receiver: &str, ip_addr_caster: &str) -> Self {
        let socket = UdpSocket::bind(ip_addr_receiver).unwrap();
        println!("{:?}", socket);
                ReceiverSocket {
            ip_addr_caster: String::from(ip_addr_caster),
            ip_addr: String::from(ip_addr_receiver),
            socket,
        }
    }
    pub fn receive_from(&self) -> Result<SerializableImage, Box<dyn std::error::Error>> {
        let mut buf = vec![0u8; MAX_UDP_PAYLOAD*10];
        println!("Dentro la receive...");

        let mut received_packets = HashMap::new();
        let mut total_packets = None;

        loop {
            let received_bytes = self.socket.recv(&mut buf).unwrap();
            buf.truncate(received_bytes);

            // Estrarre header
            let packet_num = u32::from_be_bytes(buf[0..4].try_into()?);
            let total_packets_from_header = u32::from_be_bytes(buf[4..8].try_into()?);

            total_packets.get_or_insert(total_packets_from_header);

            // Salva il pacchetto ricevuto
            received_packets.insert(packet_num, buf[8..].to_vec());

            println!(
                "Ricevuto pacchetto {} di {}",
                packet_num + 1,
                total_packets.unwrap()
            );

            // Verifica se tutti i pacchetti sono stati ricevuti
            if let Some(total) = total_packets {
                if received_packets.len() == total as usize {
                    break;
                }
            }

            buf.clear();
        }

        // Ricostruisci i dati originali
        let mut compressed_data = Vec::new();
        for i in 0..total_packets.unwrap() {
            compressed_data.extend(&received_packets[&i]);
        }

        // Decomprimi i dati
        let mut decoder = GzDecoder::new(&compressed_data[..]);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;

        // Deserializza il frame
        let deserialized_image: SerializableImage = serde_cbor::from_slice(&decompressed_data)?;

        println!("Ricostruito il frame originale");

        Ok(deserialized_image)
    }
    pub fn register_with_caster(&self) {
        let message = RegistrationMessage {
            ip: self.ip_addr.split(':').next().unwrap().to_string(),
            port: self.ip_addr.split(':').nth(1).unwrap().parse().unwrap(),
        };

        let caster_ip = self.ip_addr_caster.clone();
        let serialized = serde_cbor::to_vec(&message).unwrap();
        self.socket.send_to(&serialized, caster_ip).unwrap();
    }
}
