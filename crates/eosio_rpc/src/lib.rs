#![warn(
    clippy::all,
    clippy::complexity,
    clippy::style,
    clippy::perf,
    clippy::nursery,
    clippy::cargo
)]

pub mod chain;
pub mod history;
pub mod net;
pub mod producer;

mod client;
mod error;

pub use self::client::*;
pub use self::error::*;

#[macro_export]
macro_rules! builder {
    ($path:expr, $params:ty, $output:ty) => {
        impl $params {
            pub fn fetch(
                &self,
                client: &crate::Client,
            ) -> impl futures::future::Future<Item = $output, Error = crate::Error> {
                client.fetch::<$output, $params>($path, self.clone())
            }
        }
    };
}

// mod builder {
//     use crate::client::Client;
//     use crate::error::Error;
//     use futures::future::Future;
//     use serde::{Deserialize, Serialize};

//     pub trait Builder: Serialize {
//         const PATH: &'static str;

//         type Output: 'static + for<'de> Deserialize<'de>;

//         fn fetch(&self, client: &Client) -> Box<Future<Item = Self::Output, Error = Error>> {
//             Box::new(client.fetch(Self::PATH, Some(self)))
//         }
//     }
// }

// pub use self::builder::*;
