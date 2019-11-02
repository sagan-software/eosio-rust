use structopt::StructOpt;

/// Retrieve version information
#[derive(StructOpt, Debug)]
pub enum Version {
    /// Retrieve version information of the client
    Client,
}
