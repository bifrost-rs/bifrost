use std::net::SocketAddr;

pub enum Candidate {
    Host(SocketAddr),
    ServerReflexive(SocketAddr),
    PeerReflexive(SocketAddr),
}
