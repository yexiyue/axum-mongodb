use quote::quote;
use syn::{LitStr, Result, Token};

#[derive(Debug)]
pub struct SingleIndex {
    pub name: Option<String>,
    pub unique: bool,
    pub field_name: String,
}

impl SingleIndex {
    pub fn builder() -> SingleIndexBuilder {
        SingleIndexBuilder {
            name: None,
            unique: None,
            field_name: None,
        }
    }

    pub fn create_index(&self) -> proc_macro2::TokenStream {
        let field_name = &self.field_name;
        let name = &self.name;
        let unique = self.unique;
        if self.name.is_some() {
            quote! {
                axum_mongodb::CreateIndexOptions{
                    keys:doc!{
                        #field_name:1
                    },
                    name:Some(#name.to_string()),
                    unique:#unique,
                }
            }
        } else {
            quote! {
                axum_mongodb::CreateIndexOptions{
                    keys:doc!{
                        #field_name:1
                    },
                    name:None,
                    unique:#unique,
                }
            }
        }
    }
}

pub struct SingleIndexBuilder {
    pub name: Option<String>,
    pub unique: Option<bool>,
    pub field_name: Option<String>,
}

impl SingleIndexBuilder {
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
            return Ok(());
        })?;
        Ok(self)
    }
    pub fn build(&self) -> SingleIndex {
        SingleIndex {
            name: self.name.clone(),
            unique: self.unique.unwrap_or(false),
            field_name: self.field_name.clone().expect("field_name is required"),
        }
    }
}
