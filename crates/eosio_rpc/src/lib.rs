#![feature(async_await, await_macro, futures_api, pin)]

pub mod chain;
pub mod history;
pub mod net;
pub mod producer;

mod client;
mod error;

pub use self::client::*;
pub use self::error::*;

mod builder {
    use crate::client::Client;
    use crate::error::Error;
    use futures::future::Future;
    use serde::{Deserialize, Serialize};

    pub trait Builder: Serialize {
        const PATH: &'static str;

        type Output: 'static + for<'de> Deserialize<'de>;

        fn fetch<C: Client>(&self, client: &C) -> Box<Future<Item = Self::Output, Error = Error>> {
            Box::new(client.fetch(Self::PATH, Some(self)))
        }
    }
}

pub use self::builder::*;
