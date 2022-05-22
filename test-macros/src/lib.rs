extern crate proc_macro;

#[proc_macro_attribute]
pub fn logging_test(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Get the function to which the attribute was applied
    let func = syn::parse_macro_input!(input as syn::ItemFn);
    // Get the name of the function
    let func_name = func.sig.ident.clone();
    // Get the body of the function
    let func_body = func.block;
    // Create the output
    let tokens = quote::quote! {
        // Re-define the test function
        #[test]
        pub fn #func_name() {
            // Initialize logging
            log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
            // Run the body normally
            #func_body
        }
    };
    // Return the tokens as a token stream
    proc_macro::TokenStream::from(tokens)
}

#[proc_macro_attribute]
pub fn logging_test_async(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Get the function to which the attribute was applied
    let func = syn::parse_macro_input!(input as syn::ItemFn);
    // Get the name of the function
    let func_name = func.sig.ident.clone();
    // Get the body of the function
    let func_body = func.block;
    // Create the output
    let tokens = quote::quote! {
        // Re-define the test function
        #[tokio::test]
        pub async fn #func_name() {
            // Initialize logging
            log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
            // Run the body normally
            #func_body
        }
    };
    // Return the tokens as a token stream
    proc_macro::TokenStream::from(tokens)
}
