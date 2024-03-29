use crate::{Attr, Field};

#[derive(Debug)]
pub struct Data {
    pub attrs: Vec<Attr>,
    pub ident: syn::Ident,
    pub fields: Vec<Field>,
}

impl From<&syn::DeriveInput> for Data {
    fn from(inp: &syn::DeriveInput) -> Self {
        Self {
            attrs: inp
                .attrs
                .iter()
                .filter_map(|a| {
                    if a.path().is_ident("display") {
                        Some(a.into())
                    } else {
                        None
                    }
                })
                .collect(),
            ident: inp.ident.clone(),
            fields: match inp.data {
                syn::Data::Struct(ref s) => s.fields.iter().map(From::from).collect(),
                _ => panic!("Can't DisplayAction on non-structs"),
            },
        }
    }
}
