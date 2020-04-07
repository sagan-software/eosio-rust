#![warn(
    clippy::all,
    clippy::complexity,
    clippy::style,
    clippy::perf,
    clippy::nursery,
    clippy::cargo
)]

pub mod chain;

mod client;
mod clients;
mod error;

pub use self::{client::*, clients::*, error::*};

#[macro_export]
macro_rules! builder {
    ($path:expr, $params:ty, $output:ty) => {
        impl $params {
            pub fn fetch<C: crate::Client>(
                self,
                client: &C,
            ) -> impl std::future::Future<Output = Result<$output, crate::Error>> {
                client.fetch::<$output, $params>($path, self)
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

//         fn fetch(&self, client: &Client) -> Box<Future<Item = Self::Output,
// Error = Error>> {             Box::new(client.fetch(Self::PATH, Some(self)))
//         }
//     }
// }

// pub use self::builder::*;
