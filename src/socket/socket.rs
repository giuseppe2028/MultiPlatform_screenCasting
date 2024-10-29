use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

struct CasterSocket{
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
    fn send_to_receivers(&self,data:&[u8]){
        let socket = self.socket.lock().unwrap();
        for address in &self.receiver_sockets{
            let _ = socket.send_to(data,address);
        }
    }
}


struct ReceiverSocket{
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
    fn receive_from(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 1024];
        let socket = self.socket.lock().unwrap();

        let received_bytes = socket.recv(&mut buf).unwrap();
        buf.truncate(received_bytes);

        buf
    }

}