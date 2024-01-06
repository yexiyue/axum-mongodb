use quote::quote;
use syn::{parse::Parse, LitStr, Result, Token};

#[derive(Debug)]
pub struct CompoundIndex {
    pub name: Option<String>,
    pub unique: bool,
    pub field_name: String,
    pub other_fields: Vec<String>,
}

impl CompoundIndex {
    pub fn builder() -> CompoundIndexBuilder {
        CompoundIndexBuilder {
            name: None,
            unique: None,
            field_name: None,
            other_fields: Vec::new(),
        }
    }

    pub fn create_index(&self) -> proc_macro2::TokenStream {
        let field_name = &self.field_name;
        let name = &self.name;
        let unique = self.unique;
        let other_fields = &self.other_fields;
        if self.name.is_some() {
            quote! {
                axum_mongodb::CreateIndexOptions{
                    keys:doc!{
                        #field_name:1,
                        #(#other_fields:1),*
                    },
                    name:Some(#name.to_string()),
                    unique:#unique,
                }
            }
        } else {
            quote! {
                axum_mongodb::CreateIndexOptions{
                    keys:doc!{
                        #field_name:1,
                        #(#other_fields:1),*
                    },
                    name:None,
                    unique:#unique,
                }
            }
        }
    }
}

pub struct CompoundIndexBuilder {
    pub name: Option<String>,
    pub unique: Option<bool>,
    pub field_name: Option<String>,
    pub other_fields: Vec<String>,
}

impl CompoundIndexBuilder {
    pub fn set_field_name(&mut self, field_name: &str) -> &mut Self {
        self.field_name = Some(field_name.to_string());
        self
    }
    pub fn parse_attr(&mut self, attr: &syn::Attribute) -> Result<&mut Self> {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("unique") {
                self.unique = Some(true);
                return Ok(());
            }
            if meta.path.is_ident("name") {
                meta.input.parse::<Token![=]>()?;
                let value = meta.input.parse::<LitStr>()?;
                self.name = Some(value.value());
                return Ok(());
            }
            if meta.path.is_ident("other_fields") {
                let count;
                syn::parenthesized!(count in meta.input);
                let fields = count.parse_terminated(syn::Ident::parse, Token![,])?;
                for field in fields {
                    self.other_fields.push(field.to_string());
                }
            }
            return Ok(());
        })?;
        Ok(self)
    }
    pub fn build(&self) -> CompoundIndex {
        CompoundIndex {
            name: self.name.clone(),
            unique: self.unique.unwrap_or(false),
            field_name: self.field_name.clone().expect("field_name is required"),
            other_fields: self.other_fields.clone(),
        }
    }
}
