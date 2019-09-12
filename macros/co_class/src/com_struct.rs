use proc_macro2::TokenStream as HelperTokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Fields, Ident, ItemStruct};

/// The actual COM object that wraps around the Init struct.
/// Structure of the object:
/// pub struct _ {
///     ..base interface vpointers..
///     ..ref count..
///     ..init struct..
/// }
pub fn generate(
    aggr_map: &HashMap<Ident, Vec<Ident>>,
    base_interface_idents: &[Ident],
    struct_item: &ItemStruct,
) -> HelperTokenStream {
    let struct_ident = &struct_item.ident;
    let vis = &struct_item.vis;

    let bases_interface_idents = base_interface_idents.iter().map(|base| {
        let field_ident = macro_utils::vptr_field_ident(&base);
        quote!(#field_ident: <dyn #base as com::ComInterface>::VPtr)
    });

    let ref_count_ident = macro_utils::ref_count_ident();

    let fields = match &struct_item.fields {
        Fields::Named(f) => &f.named,
        _ => panic!("Found non Named fields in struct."),
    };

    let aggregates = aggr_map.iter().map(|(aggr_field_ident, _)| {
        quote!(
            #aggr_field_ident: *mut <dyn com::IUnknown as com::ComInterface>::VPtr
        )
    });

    quote!(
        #[repr(C)]
        #vis struct #struct_ident {
            #(#bases_interface_idents,)*
            #ref_count_ident: u32,
            #(#aggregates,)*
            #fields
        }
    )
}