use crate::error::Error;
use crate::Client;
use futures::compat::Future01CompatExt;
use futures::future::FutureExt;
use hyper::rt::{Future, Stream};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

pub struct HyperClient {
    node: String,
}

impl HyperClient {
    pub fn new(node: &str) -> Self {
        Self {
            node: node.to_owned(),
        }
    }
}

impl Client for HyperClient {
    fn node(&self) -> &str {
        &self.node
    }

    fn fetch<Output, Params>(
        &self,
        path: &str,
        params: Params,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Output, Error>> + Send>>
    where
        Output: 'static + for<'b> Deserialize<'b> + Send,
        Params: Serialize,
    {
        let https = HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);

        let mut url = self.node.to_owned();
        url.push_str(path);

        let uri: hyper::Uri = url.parse().unwrap();

        let json = serde_json::to_string(&params).unwrap();
        let mut req = hyper::Request::new(hyper::Body::from(json));
        *req.method_mut() = hyper::Method::POST;
        *req.uri_mut() = uri;
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            hyper::header::HeaderValue::from_static("application/json"),
        );

        client
            .request(req)
            .and_then(|res| res.into_body().concat2())
            .from_err::<Error>()
            .and_then(|body| {
                let s = std::str::from_utf8(&body).unwrap();
                println!("!!!!!!!!!!!!!!!!!!! {}", s);
                let res = serde_json::from_slice(&body)?;
                Ok(res)
            })
            .from_err()
            .compat()
            .boxed()
    }
}
