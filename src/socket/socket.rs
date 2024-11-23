use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use bincode::Encode;
use serde::{Deserialize, Serialize};
use xcap::image::RgbaImage;
#[derive(Serialize,Deserialize,Encode)] // Deriviamo il trait Serialize per rendere l'immagine serializzabile
struct SerializableImage {
    width: u32,
    height: u32,
    data: Vec<u8>, // Memorizziamo i dati dei pixel come un array di byte
}

pub(crate) struct CasterSocket{
    ip_addr:String,
    socket:Arc<Mutex<UdpSocket>>,
    receiver_sockets:Vec<String>
}

impl CasterSocket{
    fn new(ip_addr:&str)->Self{
        CasterSocket{
            receiver_sockets: vec![],
            ip_addr:String::from(ip_addr),
            socket:Arc::new(Mutex::new(UdpSocket::bind(ip_addr).unwrap()))
        }
    }
    fn send_to_receivers(&self,frame:RgbaImage){
        let serializable_image = SerializableImage {
            width: frame.width(),
            height: frame.height(),
            data: frame.into_raw(),
        };

        let serialized = serde_cbor::to_vec(&serializable_image).unwrap();

        let socket = self.socket.lock().unwrap();
        for address in &self.receiver_sockets{
            let _ = socket.send_to(&serialized,address);
        }
    }
}


pub struct ReceiverSocket{
    ip_addr_caster:String,
    ip_addr:String,
    socket:Arc<Mutex<UdpSocket>>
}

impl ReceiverSocket{
    fn new(ip_addr_receiver:&str,ip_addr_caster:&str)->Self{
        ReceiverSocket{
            ip_addr_caster:String::from(ip_addr_caster),
            ip_addr:String::from(ip_addr_receiver),
            socket:Arc::new(Mutex::new(UdpSocket::bind(ip_addr_receiver).unwrap()))
        }
    }
    fn receive_from(&self) -> Result<SerializableImage,Box<dyn std::error::Error>> {
        let mut buf = vec![0u8; 1024];
        let socket = self.socket.lock().unwrap();

        let received_bytes = socket.recv(&mut buf).unwrap();
        buf.truncate(received_bytes);

        let deserialized_image: SerializableImage = serde_cbor::from_slice(&buf).unwrap();

        Ok(deserialized_image)
    }

}