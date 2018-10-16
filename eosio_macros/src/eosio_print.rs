use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::Expr;

pub fn expand(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    let args = parser.parse(input).unwrap();
    let mut prints = quote!();
    for i in args.iter() {
        prints = quote! {
            #prints
            ::eosio::Print::print(&#i);
        };
    }
    TokenStream::from(quote!(#prints))
}
