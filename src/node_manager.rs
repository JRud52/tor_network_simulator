
use node::Node;

struct NodeManager {
    nodes: Vec<Node>
}

impl NodeManager {
    fn new() -> NodeManager {
        NodeManager {
            nodes: Vec::new()
        }
    }
}