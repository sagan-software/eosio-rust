use eosio::*;

#[eosio::action]
pub fn post(
    account: AccountName,
    post_num: u32,
    title: String,
    content: String,
    reply_to_account: AccountName,
    reply_to_post_num: u32,
    certify: bool,
) {
    require_auth(account);
    assert!(
        title.len() < 128,
        "Title should be less than 128 characters long."
    );
    assert!(
        content.is_empty(),
        "Content should be more than 0 characters long."
    );
    assert!(
        content.len() < 1024 * 1024 * 10,
        "Content should be less than 10 KB long."
    );
    assert!(
        post_num > 0,
        "Post number should be greater than 0 to post."
    );
    if reply_to_account.as_u64() == 0 {
        assert!(reply_to_post_num == 0, "If reply_to_account is not set, reply_to_post_num should not be set.");
    } else {
        assert!(
            is_account(reply_to_account),
            "reply_to_account must be a valid account."
        );
        assert!(
            title.is_empty(),
            "If the post is a reply, there should not be a title."
        );
    }
}

#[eosio::action]
pub fn remove(account: AccountName, post_num: u32) {
    require_auth(account);
    assert!(
        post_num > 0,
        "Post number should be greater than 0 to remove."
    );
}

eosio::abi!(post, remove);
