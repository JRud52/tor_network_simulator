
use packet::Packet;

pub struct Node {
    pub queue: Vec<Packet>
}

impl Node {
    fn new() -> Node {
        Node {
            queue: Vec::new()
        }
    }
}