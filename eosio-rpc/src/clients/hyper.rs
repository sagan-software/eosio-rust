use crate::error::Error;
use crate::Client;
use hyper;
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

pub struct HyperClient {
    node: String,
}

impl Client for HyperClient {
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
        let https = HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);

        let mut url = self.node.to_owned();
        url.push_str(path);

        let uri: hyper::Uri = url.parse().unwrap();

        let json = serde_json::to_string(&params).unwrap();
        let mut req = hyper::Request::new(hyper::Body::from(json));
        *req.method_mut() = hyper::Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            hyper::header::HeaderValue::from_static("application/json"),
        );

        let future = client
            .request(req)
            .and_then(|res| res.into_body().concat2())
            .from_err::<Error>()
            .and_then(|body| {
                let res = serde_json::from_slice(&body)?;
                Ok(res)
            })
            .from_err();

        Box::new(future)
    }
}
