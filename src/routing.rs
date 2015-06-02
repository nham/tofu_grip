use host;
use key;
use peer;
use routing;

use std::time;

struct IpfsDht {
    host: host::Host,
    id: peer::ID,
    peer_store: peer::PeerStore,
    routing_table: routing::RoutingTable,
}

// "the routing module interface. It is implemented by things like DHTs"
trait IpfsRouting {
    fn put_val(&self, key: key::Key, data: &[u8]) -> Result<(), String>;
    fn get_val(&self, key: key::Key) -> Result<&[u8], String>;
    fn provide(&self, key: key::Key) -> Result<(), String>;
    fn find_peer(&self, peer: peer::ID) -> Result<peer::PeerInfo, String>;
    fn ping(&self, peer: peer::ID) -> Result<time::Duration, String>;
}
