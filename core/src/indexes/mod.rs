mod compound;
mod multikey;
mod single;
use compound::CompoundIndex;
use multikey::MultikeyIndex;
use single::SingleIndex;
use syn::Result;
pub enum Indexes {
    Single(SingleIndex),
    Compound(CompoundIndex),
    Multikey(MultikeyIndex),
}

impl Indexes {
    pub fn parse_from_attr(attr: &syn::Attribute, field_name: &str) -> Result<Option<Self>> {
        if attr.path().is_ident("singleIndex") {
            Ok(Some(Self::Single(
                SingleIndex::builder()
                    .set_field_name(field_name)
                    .parse_attr(attr)?
                    .build(),
            )))
        } else if attr.path().is_ident("compoundIndex") {
            Ok(Some(Self::Compound(
                CompoundIndex::builder()
                    .set_field_name(field_name)
                    .parse_attr(attr)?
                    .build(),
            )))
        } else if attr.path().is_ident("multikeyIndex") {
            Ok(Some(Self::Multikey(
                MultikeyIndex::builder()
                    .set_field_name(field_name)
                    .parse_attr(attr)?
                    .build(),
            )))
        } else {
            Ok(None)
        }
    }
    pub fn create_index(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Single(single) => single.create_index(),
            Self::Compound(compound) => compound.create_index(),
            Self::Multikey(multikey) => multikey.create_index(),
        }
    }
}
