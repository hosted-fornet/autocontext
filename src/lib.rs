use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn autocontext(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Example transformation: wrap the function body in a context block
    let fn_body = &input_fn.block;
    let fn_sig = &input_fn.sig;
    let fn_attrs = &input_fn.attrs;
    let fn_vis = &input_fn.vis;

    let transformed = quote! {
        #(#fn_attrs)* #fn_vis #fn_sig {
            let result = (|| -> anyhow::Result<_> { #fn_body })();
            match result {
                Ok(val) => Ok(val),
                Err(e) => Err(anyhow::anyhow!(e).context(format!("Error occurred in function at {}:{}", file!(), line!() + 1))),
            }
        }
    };

    transformed.into()
}
