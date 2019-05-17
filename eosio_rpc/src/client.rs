use crate::error::Error;
use futures::future::Future;
use serde::{Deserialize, Serialize};

pub trait Client {
    fn node(&self) -> &str;

    fn fetch<Output, Params>(
        &self,
        path: &str,
        params: Params,
    ) -> Box<dyn Future<Item = Output, Error = Error>>
    where
        Output: 'static + for<'b> Deserialize<'b>,
        Params: Serialize;
}
