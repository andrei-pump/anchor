use crate::Program;
use heck::SnakeCase;
use quote::quote;

pub fn generate(program: &Program) -> proc_macro2::TokenStream {
    let mut account_paths = std::collections::HashSet::new();

    // Go through instruction accounts.
    for ix in &program.ixs {
        let path = ix.anchor_path.clone();

        // Get the segments of the path
        let mut segments = path.segments;

        // Modify the last segment
        if let Some(last_segment) = segments.last_mut() {
            // let input = format!("__client_accounts_{}", last_segment.ident);
            // let new_ident = syn::parse_macro_input!(input as syn::Ident);
            let new_ident = syn::Ident::new(
                &format!(
                    "__client_accounts_{}",
                    last_segment.ident.to_string().to_snake_case()
                ),
                last_segment.ident.span(),
            );
            *last_segment = syn::PathSegment {
                ident: new_ident,
                arguments: last_segment.arguments.clone(),
            };
        }

        // Construct the new Path
        let new_path = syn::Path {
            leading_colon: path.leading_colon,
            segments,
        };

        account_paths.insert(new_path);
    }

    // Build the tokens from all accounts
    let account_structs: Vec<proc_macro2::TokenStream> = account_paths
        .iter()
        .map(|path| {
            quote! {
                pub use crate::#path::*;
            }
        })
        .collect();

    // TODO: calculate the account size and add it as a constant field to
    //       each struct here. This is convenient for Rust clients.

    quote! {
        /// An Anchor generated module, providing a set of structs
        /// mirroring the structs deriving `Accounts`, where each field is
        /// a `Pubkey`. This is useful for specifying accounts for a client.
        pub mod accounts {
            #(#account_structs)*
        }
    }
}
