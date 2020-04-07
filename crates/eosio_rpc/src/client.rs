use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};

pub trait Client {
    fn node(&self) -> &str;

    fn fetch<Output, Params>(
        &self,
        path: &str,
        params: Params,
    ) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + Send>>
    where
        Output: 'static + for<'b> Deserialize<'b> + Send,
        Params: Serialize;
}
