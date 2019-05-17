use crate::error::Error;
use crate::Client;
use futures::future::{self, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

pub struct WebSysClient {
    node: String,
}

impl Client for WebSysClient {
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
        let mut url = self.node.to_owned();
        url.push_str(path);

        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);

        let body_string = serde_json::to_string(&params).unwrap();
        opts.body(Some(&JsValue::from_str(body_string.as_str())));

        let request = Request::new_with_str_and_init(url.as_str(), &opts).unwrap();
        let window = web_sys::window().expect("no window object available");
        let request_promise = window.fetch_with_request(&request);

        let fut = JsFuture::from(request_promise)
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
        Box::new(fut)
    }
}
