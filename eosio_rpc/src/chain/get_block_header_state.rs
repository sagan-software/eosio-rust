const PATH: &str = "/v1/chain/get_block_header_state";

struct Params {
    block_num_or_id: String,
}
