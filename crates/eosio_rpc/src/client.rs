use crate::error::Error;
use futures::future::Future;
use serde::{Deserialize, Serialize};

pub trait Client {
    fn fetch<Output, Params>(
        &self,
        path: &str,
        params: Option<Params>,
    ) -> Box<Future<Item = Output, Error = Error>>
    where
        Output: 'static + for<'a> Deserialize<'a>,
        Params: Serialize;
}

#[cfg(feature = "use-hyper")]
mod hyper {
    use crate::error::Error;
    use hyper::rt::{self, Future, Stream};
    use hyper::Client;
    use hyper_tls::HttpsConnector;
    use serde::{Deserialize, Serialize};

    pub struct HyperClient<'a> {
        node: &'a str,
    }

    impl<'a> HyperClient<'a> {
        pub fn new(node: &'a str) -> Result<Self, String> {
            Ok(HyperClient { node })
        }
    }

    impl<'a> super::Client for HyperClient<'a> {
        fn fetch<Output, Params>(
            &self,
            path: &str,
            params: Option<Params>,
        ) -> Box<Future<Item = Output, Error = Error>>
        where
            Output: 'static + for<'b> Deserialize<'b>,
            Params: Serialize,
        {
            let https = HttpsConnector::new(4).unwrap();
            let client = Client::builder().build::<_, hyper::Body>(https);

            let mut url = node.to_owned();
            url.push_str(path);

            let url = url.parse().unwrap();

            let future = client
                // Fetch the url...
                .get(url)
                // And then, if we get a response back...
                .and_then(|res| {
                    // asynchronously concatenate chunks of the body
                    res.into_body().concat2()
                })
                .from_err::<Error>()
                // use the body after concatenation
                .and_then(|body| {
                    // try to parse as json with serde_json
                    let users = serde_json::from_slice(&body)?;

                    Ok(users)
                })
                .from_err();
            Box::new(future)
        }
    }
}

#[cfg(feature = "use-hyper")]
pub use self::hyper::*;

#[cfg(feature = "use-stdweb")]
mod stdweb {
    pub struct StdwebClient {}
}

#[cfg(feature = "use-stdweb")]
pub use self::stdweb::*;

#[cfg(feature = "use-web-sys")]
mod web_sys {
    use crate::error::Error;
    use futures::future::{self, Future};
    use js_sys::Promise;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response, Window};

    pub struct WebSysClient<'a> {
        node: &'a str,
        window: Window,
    }

    impl<'a> WebSysClient<'a> {
        pub fn new(node: &'a str) -> Result<Self, String> {
            match ::web_sys::window() {
                Some(window) => Ok(WebSysClient { node, window }),
                None => Err("no window object available".to_string()),
            }
        }
    }

    impl<'a> super::Client for WebSysClient<'a> {
        fn fetch<Output, Params>(
            &self,
            path: &str,
            params: Option<Params>,
        ) -> Box<Future<Item = Output, Error = Error>>
        where
            Output: 'static + for<'b> Deserialize<'b>,
            Params: Serialize,
        {
            let mut url = self.node.to_owned();
            url.push_str(path);

            let mut opts = RequestInit::new();
            opts.method("POST");
            opts.mode(RequestMode::Cors);

            if let Some(params) = params {
                match ::serde_json::to_string(&params) {
                    Ok(s) => opts.body(Some(&JsValue::from_str(s.as_str()))),
                    Err(error) => return Box::new(future::err(Error::BadRequestJson(error))),
                };
            }

            let request = match Request::new_with_str_and_init(url.as_str(), &opts) {
                Ok(r) => r,
                Err(_) => return Box::new(future::err(Error::BadRequest)),
            };

            let request_promise = self.window.fetch_with_request(&request);

            let future = JsFuture::from(request_promise)
                .map_err(|_| Error::BadResponse)
                .and_then(|resp_value| {
                    assert!(resp_value.is_instance_of::<Response>());
                    let resp = resp_value.dyn_into::<Response>().unwrap();
                    resp.json().map_err(|_| Error::BadResponse)
                })
                .and_then(|json_value: Promise| {
                    JsFuture::from(json_value).map_err(|_| Error::BadResponse)
                })
                .and_then(|json| match json.into_serde::<Output>() {
                    Ok(output) => future::ok(output),
                    Err(err) => future::err(Error::BadResponseJson(err)),
                });
            Box::new(future)
        }
    }
}

#[cfg(feature = "use-web-sys")]
pub use self::web_sys::*;
