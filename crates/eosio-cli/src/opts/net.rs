use structopt::StructOpt;

/// Interact with local p2p network connections
#[derive(StructOpt, Debug)]
pub enum Net {
    /// start a new connection to a peer
    Connect(Connect),
    /// close an existing connection
    Disconnect(Disconnect),
    /// status of existing connection
    Status(Status),
    /// status of all existing peers
    Peers,
}

/// start a new connection to a peer
#[derive(StructOpt, Debug)]
pub struct Connect {
    /// The hostname:port to connect to.
    pub host: String,
}

/// close an existing connection
#[derive(StructOpt, Debug)]
pub struct Disconnect {
    /// The hostname:port to disconnect from.
    pub host: String,
}

/// status of existing connection
#[derive(StructOpt, Debug)]
pub struct Status {
    /// The hostname:port to query status of connection
    pub host: String,
}
