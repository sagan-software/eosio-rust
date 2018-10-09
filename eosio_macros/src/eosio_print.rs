use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, Lit};

pub fn expand(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    let args = parser.parse(input).unwrap();
    let mut prints = quote!();
    for i in args.iter() {
        let mut printable = quote!(#i);
        if let Expr::Lit(ref lit) = *i {
            if let Lit::Str(ref strlit) = lit.lit {
                printable = quote!(::eosio::c!(#strlit));
            }
        }
        prints = quote! {
            #prints
            ::eosio::Printable::print(&#printable);
        };
    }
    TokenStream::from(quote!(#prints))
}
