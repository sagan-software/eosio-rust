const PATH: &str = "/v1/chain/get_producers";

struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lower_bound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<bool>,
}
