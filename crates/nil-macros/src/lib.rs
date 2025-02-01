mod derive;

use proc_macro::TokenStream;

#[proc_macro_derive(Unit)]
pub fn derive_unit(input: TokenStream) -> TokenStream {
  derive::impl_unit(&syn::parse(input).unwrap())
}
