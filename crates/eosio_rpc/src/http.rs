use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub fn get<Output>(node: &str, path: &str) -> impl Future<Item = Output, Error = crate::Error>
where
    Output: for<'a> Deserialize<'a>,
{
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let mut url = node.to_owned();
    url.push_str(path);
    let request = Request::new_with_str_and_init(url.as_str(), &opts).unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp = resp_value.dyn_into::<Response>().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            let out = json.into_serde::<Output>().unwrap();

            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(out)
        })
        .map_err(crate::Error::JsError)
}

pub fn post<Output, Params>(
    node: &str,
    path: &str,
    params: Params,
) -> impl Future<Item = Output, Error = crate::Error>
where
    Output: for<'a> Deserialize<'a>,
    Params: Serialize,
{
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    match ::serde_json::to_string(&params) {
        Ok(s) => opts.body(Some(&JsValue::from_str(s.as_str()))),
        Err(_) => panic!("balls"),
        // Err(error) => return future::err(Error::BadRequestJson(error)).fuse(),
    };

    let mut url = node.to_owned();
    url.push_str(path);
    let request = match Request::new_with_str_and_init(url.as_str(), &opts) {
        Ok(r) => r,
        Err(_) => panic!("balls"),
        // Err(_) => return (future::err(Error::BadRequest).fuse()),
    };

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp = resp_value.dyn_into::<Response>().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            let info = json.into_serde::<Output>().unwrap();

            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(info)
        })
        .map_err(crate::Error::JsError)
}
