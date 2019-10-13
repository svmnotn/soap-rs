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
        match self {
            Attr::Group(_) => true,
            _ => false,
        }
    }

    pub fn as_group(&self) -> Group {
        match self {
            Attr::Group(v) => v.clone(),
            _ => panic!(),
        }
    }

    pub fn is_rename(&self) -> bool {
        match self {
            Attr::Rename(_) => true,
            _ => false,
        }
    }

    pub fn as_rename(&self) -> syn::LitStr {
        match self {
            Attr::Rename(v) => v.clone(),
            _ => panic!(),
        }
    }

    pub fn is_extra(&self) -> bool {
        match self {
            Attr::Extra(_) => true,
            _ => false,
        }
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
        let attr = inp.parse_meta().unwrap();
        let id = attr.path().get_ident().unwrap();
        if id == "display" {
            match attr {
                syn::Meta::List(ref l) => match l.nested.len() {
                    1 => {
                        if let syn::NestedMeta::Meta(ref m) = l.nested.first().unwrap() {
                            if let syn::Meta::NameValue(ref nv) = m {
                                let id = nv.path.get_ident().unwrap();
                                if id == "group" {
                                    if let syn::Lit::Str(ref s) = nv.lit {
                                        Self::Group(Group {
                                            name: s.clone(),
                                            id: None,
                                        })
                                    } else {
                                        unimplemented!()
                                    }
                                } else if id == "rename" {
                                    if let syn::Lit::Str(ref s) = nv.lit {
                                        Self::Rename(s.clone())
                                    } else {
                                        unimplemented!()
                                    }
                                } else if id == "extra" {
                                    if let syn::Lit::Str(ref s) = nv.lit {
                                        Self::Extra(s.clone())
                                    } else {
                                        unimplemented!()
                                    }
                                } else {
                                    unimplemented!()
                                }
                            } else {
                                unimplemented!()
                            }
                        } else {
                            unimplemented!()
                        }
                    }
                    2 => {
                        let group = if let syn::NestedMeta::Meta(ref m) = l.nested.first().unwrap()
                        {
                            if let syn::Meta::NameValue(ref nv) = m {
                                let id = nv.path.get_ident().unwrap();
                                if id == "group" {
                                    if let syn::Lit::Str(ref s) = nv.lit {
                                        s.clone()
                                    } else {
                                        unimplemented!()
                                    }
                                } else {
                                    unimplemented!()
                                }
                            } else {
                                unimplemented!()
                            }
                        } else {
                            unimplemented!()
                        };
                        let id = if let syn::NestedMeta::Meta(ref m) = l.nested.last().unwrap() {
                            if let syn::Meta::NameValue(ref nv) = m {
                                let id = nv.path.get_ident().unwrap();
                                if id == "id" {
                                    if let syn::Lit::Int(ref i) = nv.lit {
                                        i.clone()
                                    } else {
                                        unimplemented!()
                                    }
                                } else {
                                    unimplemented!()
                                }
                            } else {
                                unimplemented!()
                            }
                        } else {
                            unimplemented!()
                        };
                        Self::Group(Group {
                            name: group,
                            id: Some(id),
                        })
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        } else {
            unimplemented!()
        }
    }
}
