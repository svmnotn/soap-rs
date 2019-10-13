use crate::{Attr, Group};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub extras: Vec<syn::LitStr>,
    pub groups: Vec<Group>,
    pub rename: Option<syn::LitStr>,
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

impl Field {
    pub fn is_option(&self) -> bool {
        match self.ty {
            syn::Type::Path(ref t) => {
                if let Some(ty) = t.path.segments.first() {
                    ty.ident == "Option"
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl From<&syn::Field> for Field {
    fn from(inp: &syn::Field) -> Self {
        let attr: Vec<Attr> = inp.attrs.iter().map(From::from).collect();
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
            ty: inp.ty.clone(),
        }
    }
}
