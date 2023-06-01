use crate::{Attr, Group};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub extras: Vec<syn::LitStr>,
    pub groups: Vec<Group>,
    pub rename: Option<syn::LitStr>,
    pub ident: syn::Ident,
    pub is_option: bool,
}

impl From<&syn::Field> for Field {
    fn from(inp: &syn::Field) -> Self {
        let attr: Vec<Attr> = inp
            .attrs
            .iter()
            .filter_map(|a| {
                if a.path().is_ident("display") {
                    Some(a.into())
                } else {
                    None
                }
            })
            .collect();
        Self {
            extras: attr
                .iter()
                .filter(|a| a.is_extra())
                .map(|a| a.as_extra())
                .collect(),
            groups: attr
                .iter()
                .filter(|a| a.is_group())
                .map(|a| a.as_group())
                .collect(),
            rename: attr.iter().find(|a| a.is_rename()).map(|a| a.as_rename()),
            ident: inp.ident.as_ref().unwrap().clone(),
            is_option: if let syn::Type::Path(ref p) = inp.ty {
                p.path
                    .segments
                    .first()
                    .map(|t| t.ident == "Option")
                    .unwrap_or_default()
            } else {
                false
            },
        }
    }
}
