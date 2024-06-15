mod error_info;
use error_info::process_error_info;
use proc_macro::TokenStream;

#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_error_code(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);
    process_error_info(input).into()
}