use syn::spanned::Spanned;
use syn::{LitInt, LitStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Group {
    pub name: syn::LitStr,
    pub id: Option<syn::LitInt>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Attr {
    Group(Group),
    Rename(syn::LitStr),
    Extra(syn::LitStr),
}

impl Attr {
    pub fn is_group(&self) -> bool {
        matches!(self, Attr::Group(_))
    }

    pub fn as_group(&self) -> Group {
        match self {
            Attr::Group(v) => v.clone(),
            _ => panic!(),
        }
    }

    pub fn is_rename(&self) -> bool {
        matches!(self, Attr::Rename(_))
    }

    pub fn as_rename(&self) -> syn::LitStr {
        match self {
            Attr::Rename(v) => v.clone(),
            _ => panic!(),
        }
    }

    pub fn is_extra(&self) -> bool {
        matches!(self, Attr::Extra(_))
    }

    pub fn as_extra(&self) -> syn::LitStr {
        match self {
            Attr::Extra(v) => v.clone(),
            _ => panic!(),
        }
    }
}

impl From<&syn::Attribute> for Attr {
    fn from(inp: &syn::Attribute) -> Self {
        fn from_attribute(inp: &syn::Attribute) -> Result<Attr, syn::Error> {
            if !inp.meta.path().is_ident("display") {
                return Err(syn::Error::new(inp.span(), "Calling `into` or `from` syn::Attribute for Attr, when the Attribute is not 'display' is not supported"));
            }

            let list = inp.meta.require_list()?;
            let mut attr = None;
            list.parse_nested_meta(|meta| {
                if meta.path.is_ident("group") {
                    let mut name = None;
                    let mut id = None;
                    if let Ok(value) = meta.value() {
                        let s: LitStr = value.parse()?;
                        name = Some(s);
                    } else {
                        meta.parse_nested_meta(|nested_meta| {
                            if nested_meta.path.is_ident("name") {
                                let value = nested_meta.value()?;
                                let s: LitStr = value.parse()?;
                                name = Some(s);
                                Ok(())
                            } else if nested_meta.path.is_ident("id") {
                                let value = nested_meta.value()?;
                                let s: LitInt = value.parse()?;
                                id = Some(s);
                                Ok(())
                            } else {
                                Err(nested_meta.error("unsupported attribute under group, only supported values are `name` and `id`"))
                            }
                        })?;
                    }

                    let Some(name) = name else {
                        return Err(meta.error(r#"group attribute requires to be either `group = "name"` or `group(name = "name")`"#));
                    };

                    attr = Some(Attr::Group(Group {
                        name,
                        id
                    }));

                    Ok(())
                } else if meta.path.is_ident("rename") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    attr = Some(Attr::Rename(s));
                    Ok(())
                } else if meta.path.is_ident("extra") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    attr = Some(Attr::Extra(s));
                    Ok(())
                } else {
                    Err(meta.error("unsupported attribute given to `display()`, supported attributes are `group`, `rename`, and `extra`"))
                }
            })?;

            attr.ok_or(syn::Error::new(inp.span(), r#"display attribute needs to be of the following forms: `display(group = "name")`, `display(group(name = "name", id = 4))`, `display(rename = "name")`, `display(extra = "name")`"#))
        }

        match from_attribute(inp) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        }
    }
}
