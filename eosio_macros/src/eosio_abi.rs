use crate::proc_macro::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::Ident;

pub fn expand(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_separated_nonempty;
    let eosio = crate::paths::eosio();
    let args = parser.parse(input).unwrap();
    let mut actions = quote!();
    for i in args.iter() {
        actions = quote! {
            #actions
            #eosio::n!(#i) => #i(),
        };
    }
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
            if action == #eosio::n!(onerror) {
                #eosio::eosio_assert(
                    code == #eosio::n!(eosio),
                    "onerror action's are only valid from the \"eosio\" system account"
                );
            }
            if code == receiver || action == #eosio::n!(onerror) {
                match action {
                    #actions
                    _ => {
                        #eosio::eosio_assert(false, "unknown action");
                    }
                }
            }
        }
    };
    TokenStream::from(quote!(#expanded))
}
