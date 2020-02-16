use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::fmt::{self, Display};
use syn::{
    parse::{Error as ParseError, Result as ParseResult},
    Attribute, Ident, Lit, LitStr, Meta,
    Meta::List,
    NestedMeta, Path,
};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const EOSIO: Symbol = Symbol("eosio");
pub const TABLE_NAME: Symbol = Symbol("table_name");
pub const SINGLETON: Symbol = Symbol("singleton");
pub const PRIMARY_KEY: Symbol = Symbol("primary_key");
pub const SECONDARY_KEY: Symbol = Symbol("secondary_key");
// pub const INDEX: Symbol = Symbol("index");
pub const CRATE_PATH: Symbol = Symbol("crate_path");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

pub fn get_eosio_meta_items(
    attr: &syn::Attribute,
) -> Result<Vec<syn::NestedMeta>, ()> {
    if attr.path != EOSIO {
        return Ok(Vec::new());
    }

    match attr.parse_meta() {
        Ok(List(meta)) => Ok(meta.nested.into_iter().collect()),
        _ => Err(()),
    }
}

/// Get the root path for types/traits.
pub fn get_root_path(attrs: &[Attribute]) -> Path {
    for meta_item in attrs.iter().flat_map(get_eosio_meta_items).flatten() {
        match meta_item {
            NestedMeta::Meta(Meta::NameValue(m)) if m.path == CRATE_PATH => {
                match m.lit {
                    Lit::Str(string) => {
                        if let Ok(path) =
                            string.parse_with(Path::parse_mod_style)
                        {
                            return path;
                        } else {
                            panic!(
                                "`#[eosio(crate_path = \"...\")]` received an \
                                 invalid path"
                            );
                        }
                    }
                    _ => {
                        panic!("invalid eosio crate path");
                    }
                }
            }
            _ => continue,
        }
    }
    LitStr::new("::eosio", Span::call_site())
        .parse_with(Path::parse_mod_style)
        .unwrap()
}

pub struct Attr<T> {
    name: Symbol,
    tokens: TokenStream,
    value: Option<T>,
}

impl<T> Attr<T> {
    pub fn none(name: Symbol) -> Self {
        Self {
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    pub fn set<A: ToTokens>(&mut self, obj: A, value: T) -> ParseResult<()> {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            Err(ParseError::new_spanned(
                tokens,
                format!("duplicate eosio attribute `{}`", self.name),
            ))
        } else {
            self.tokens = tokens;
            self.value = Some(value);
            Ok(())
        }
    }

    pub fn get(self) -> Option<T> {
        self.value
    }

    pub fn get_with_tokens(self) -> Option<(TokenStream, T)> {
        match self.value {
            Some(v) => Some((self.tokens, v)),
            None => None,
        }
    }
}

pub struct BoolAttr(Attr<()>);

impl BoolAttr {
    pub fn none(name: Symbol) -> Self {
        Self(Attr::none(name))
    }

    pub fn set_true<A: ToTokens>(&mut self, obj: A) -> ParseResult<()> {
        self.0.set(obj, ())
    }

    pub fn get(&self) -> bool {
        self.0.value.is_some()
    }
}
