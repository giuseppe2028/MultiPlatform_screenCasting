use crate::socket::socket::{CasterSocket, ReceiverSocket};

pub enum SocketController{
    CasterSocketController(CasterSocket),
    ReceiverSocketController(ReceiverSocket),
    NotDefined
}