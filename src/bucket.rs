use peer;

use std::collections::LinkedList;
use std::sync::Mutex;

#[derive(Clone)]
pub struct NodeInfo {
    pub address: net::SocketAddr,
    pub id: NodeId,
}

struct Bucket<T> {
    entries: Mutex<LinkedList<T>>,
}

impl Bucket<NodeInfo> {
    fn has(&self, id: peer::Id) -> bool {
        let entries = self.entries.lock().unwrap();
        for entry in entries.iter() {
            if entry.id == id {
                return true;
            }
        }

        false
    }
}
