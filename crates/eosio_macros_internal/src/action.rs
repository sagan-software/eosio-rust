use heck::CamelCase;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    Block, FnArg, Ident, ItemFn, LitStr, Signature,
};

pub struct ActionArgs {
    name: Option<LitStr>,
}

impl Parse for ActionArgs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.parse::<Option<LitStr>>()?;
        Ok(Self { name })
    }
}

pub struct ActionFn {
    sig: Signature,
    block: Box<Block>,
    args: ActionArgs,
}

impl ActionFn {
    pub fn new(args: ActionArgs, item: ItemFn) -> Self {
        Self {
            sig: item.sig,
            block: item.block,
            args,
        }
    }

    pub fn struct_ident(&self) -> Ident {
        let name = self.sig.ident.to_string().as_str().to_camel_case();
        Ident::new(&name, self.sig.ident.span())
    }

    pub fn action_name(&self) -> LitStr {
        if let Some(lit) = &self.args.name {
            lit.clone()
        } else {
            LitStr::new(
                self.sig.ident.to_string().as_str(),
                self.sig.ident.span(),
            )
        }
    }
}

impl ToTokens for ActionFn {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let inputs = &self.sig.inputs;
        let mut struct_fields = quote!();
        let mut assign_args = quote!();
        for input in inputs.iter() {
            match input {
                FnArg::Typed(input) => {
                    let pat = &input.pat;
                    let ty = &input.ty;
                    let ty_str = quote!(#ty).to_string();
                    let serde_attr = if ty_str == "bool" {
                        quote!(
                            #[cfg_attr(
                                feature = "serde",
                                serde(
                                    deserialize_with = "::eosio::bool_from_u8",
                                    serialize_with = "::eosio::bool_to_u8"
                                )
                            )]
                        )
                    } else {
                        quote!()
                    };
                    struct_fields = quote! {
                        #struct_fields
                        #serde_attr
                        pub #pat: #ty,
                    };
                    assign_args = quote! {
                        #assign_args
                        let #pat = self.#pat;
                    };
                }
                _ => unimplemented!(),
            }
        }
        let block = &self.block;

        let struct_ident = self.struct_ident();
        let type_ident = &self.sig.ident;
        let action_name = self.action_name();

        let expanded = quote! {
            #[derive(Clone, eosio::Read, eosio::Write, eosio::NumBytes)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
            pub struct #struct_ident {
                #struct_fields
            }

            // This makes the abi! macro work nicer
            #[allow(non_camel_case_types)]
            pub type #type_ident = #struct_ident;

            #[automatically_derived]
            impl eosio::ActionFn for #struct_ident {
                const NAME: eosio::ActionName = eosio::ActionName::new(eosio::n!(#action_name));
                fn call(self) {
                    #assign_args
                    #block
                }
            }

            // TODO: keep original function intact so it can be called like normal
            // #vis fn #ident() {
            //     let s = read_action_data::<#struct_ident>().expect("read");
            //     eosio::ActionFn::execute(s);
            // }
        };
        expanded.to_tokens(tokens);
    }
}
