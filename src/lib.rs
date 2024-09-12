use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn make_rulefor(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let name = syn::parse_macro_input!(attr as syn::Ident);
    let gen = quote! {
        impl Rule for #name {
            fn name(&self) -> &'static str {
                self.name
            }

            #item_fn
        }
    };
    gen.into()
}
