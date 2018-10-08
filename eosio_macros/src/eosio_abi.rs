use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::Ident;

pub fn expand(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_separated_nonempty;
    let args = parser.parse(input).unwrap();
    let mut actions = quote!();
    for i in args.iter() {
        actions = quote! {
            #actions
            ::eosio::macros::n!(#i) => #i(),
        };
    }
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
            if action == ::eosio::macros::n!(onerror) {
                ::eosio::assert::eosio_assert_code(
                    code == ::eosio::macros::n!(eosio),
                    ::eosio::macros::n!(badonerror)
                );
            }
            if code == receiver || action == ::eosio::macros::n!(onerror) {
                match action {
                    #actions
                    _ => {
                        ::eosio::assert::eosio_assert_code(false, n!(badaction));
                    }
                }
            }
        }
    };
    TokenStream::from(quote!(#expanded))
}
