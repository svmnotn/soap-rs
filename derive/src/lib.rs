use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use std::collections::{HashMap, HashSet};

mod attr;
use attr::{Attr, Group};
mod field;
use field::Field;
mod data;
use data::Data;

#[proc_macro_derive(DisplayAction, attributes(display))]
pub fn display_action_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input as DeriveInput);

    // Build the trait implementation
    impl_display_action(input).into()
}

fn impl_display_action(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data = Data::from(&input);
    let top_group = data
        .attrs
        .iter()
        .find(|a| a.is_group())
        .map(|a| a.as_group())
        .map(|g| g.name);
    let head = if let Some(ref g) = top_group {
        quote! {
            write!(fmt, concat!("<", #g, ">"))?;
        }
    } else {
        quote! {}
    };
    let end = if let Some(ref g) = top_group {
        quote! {
            write!(fmt, concat!("</", #g, ">"))?;
        }
    } else {
        quote! {}
    };
    let mut body: Vec<TokenStream> = Vec::new();

    let mut grouped = HashMap::new();
    for field in &data.fields {
        if field.groups.is_empty() {
            write_field(&mut body, field);
        } else {
            for group in &field.groups {
                let vals = grouped.entry(group).or_insert_with(HashSet::new);
                vals.insert(field);
            }
        }
    }

    write_top_grouped_fields(&mut body, grouped);

    let name = &data.ident;

    quote! {
        impl #impl_generics soap_rs::DisplayAction for #name #ty_generics #where_clause {
            fn fmt(&self, fmt: &mut String, _: &str) -> ::std::result::Result<(), ::std::fmt::Error> {
                use ::std::fmt::Write;

                #head
                #(#body)*
                #end

                ::std::result::Result::Ok(())
            }
        }
    }
}

fn write_field(body: &mut Vec<TokenStream>, field: &Field) {
    for extra in &field.extras {
        body.push(quote! {
            write!(fmt, #extra)?;
        });
    }

    let ident = &field.ident;
    body.push(if let Some(ref rname) = field.rename {
        quote! {
            self.#ident.fmt(fmt, #rname)?;
        }
    } else {
        quote! {
            self.#ident.fmt(fmt, stringify!(#ident))?;
        }
    });
}

fn gen_fields(grouped: &HashSet<&Field>) -> Vec<TokenStream> {
    let mut body = Vec::new();
    for field in grouped {
        if field.is_option {
            let id = &field.ident;
            body.push(quote! {
                self.#id.is_some() ||
            });
        } else {
            body.push(quote! {
                true ||
            });
        }
    }

    body.push(quote! {
        false
    });

    body
}

fn write_grouped_fields(
    body: &mut Vec<TokenStream>,
    group: &Group,
    grouped: &HashMap<&Group, HashSet<&Field>>,
) {
    let mut vals = Vec::new();
    for field in grouped.get(group).unwrap() {
        write_field(&mut vals, field);
    }
    let fields = gen_fields(grouped.get(group).unwrap());
    let name = &group.name;
    body.push(quote! {
        if #(#fields)* {
            write!(fmt, concat!("<", #name, ">"))?;
            #(#vals)*
            write!(fmt, concat!("</", #name, ">"))?;
        }
    });
}

fn write_top_grouped_fields(
    body: &mut Vec<TokenStream>,
    grouped: HashMap<&Group, HashSet<&Field>>,
) {
    for (superset, (subsets, fields)) in calculate_supersets(&grouped) {
        let mut vals = Vec::new();
        for field in grouped.get(superset).unwrap().difference(&fields) {
            write_field(&mut vals, field);
        }

        let mut sub_vals = Vec::new();
        for group in subsets {
            write_grouped_fields(&mut sub_vals, group, &grouped);
        }

        let field_testing = gen_fields(grouped.get(superset).unwrap());
        let name = &superset.name;
        body.push(quote! {
            if #(#field_testing)* {
                write!(fmt, concat!("<", #name, ">"))?;
                #(#vals)*
                #(#sub_vals)*
                write!(fmt, concat!("</", #name, ">"))?;
            }
        });
    }
}

#[allow(clippy::map_clone)]
fn calculate_supersets<'a>(
    grouped: &HashMap<&'a Group, HashSet<&'a Field>>,
) -> HashMap<&'a Group, (HashSet<&'a Group>, HashSet<&'a Field>)> {
    let mut supersets = HashMap::new();

    for (k, v) in grouped {
        let mut sup = true;
        let mut subs = HashSet::new();
        let mut sub_fields = HashSet::new();
        for (k1, v1) in grouped {
            if k != k1 && v.intersection(v1).next().is_some() {
                if v.is_superset(v1) {
                    subs.insert(*k1);
                    sub_fields = sub_fields.union(v1).map(|f| *f).collect();
                } else {
                    sup = false;
                    break;
                }
            }
        }
        if sup {
            supersets.insert(*k, (subs, sub_fields));
        }
    }

    supersets
}
