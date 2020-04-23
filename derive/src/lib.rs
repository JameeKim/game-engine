use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod system_desc;

#[proc_macro_derive(SystemDesc, attributes(system_desc))]
pub fn derive_system_desc(input: TokenStream) -> TokenStream {
    system_desc::derive(parse_macro_input!(input as DeriveInput)).into()
}
