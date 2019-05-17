use crate::error::Error;
use crate::Client;
use futures::future::{self, Future};
use serde::{Deserialize, Serialize};

pub struct StdwebClient {
    node: String,
}

impl Client for StdwebClient {
    fn node(&self) -> &str {
        &self.node
    }
    fn fetch<Output, Params>(
        &self,
        path: &str,
        params: Params,
    ) -> Box<Future<Item = Output, Error = Error>>
    where
        Output: 'static + for<'b> Deserialize<'b>,
        Params: Serialize,
    {
        // TODO
        Box::new(futures::future::err(Error::BadRequest))
    }
}
