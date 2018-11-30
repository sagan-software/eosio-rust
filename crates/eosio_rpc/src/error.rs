pub enum Error {
    BadRequestJson(::serde_json::Error),
    BadRequest,
    NoWindow,
    BadResponse,
    BadResponseJson(::serde_json::Error),
    JsError(::wasm_bindgen::JsValue),
    EosError(ErrorResponse),
}

pub struct ErrorResponse {
    code: u16,
    message: String,
    error: ErrorMessage,
}

pub struct ErrorMessage {
    code: u16,
    name: String,
    what: String,
    details: Vec<ErrorDetails>,
}

pub struct ErrorDetails {
    message: String,
    file: String,
    line_number: u32,
    method: String,
}
