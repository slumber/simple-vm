// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_attribute]
pub fn bindgen(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    let sig = &input.sig;

    if sig.inputs.len() > 0 {
        let err = "function arguments are not supported";
        return syn::Error::new_spanned(&sig, err).to_compile_error().into();
    }

    if !matches!(sig.output, syn::ReturnType::Default) {
        let err = "function outputs are not supported";
        return syn::Error::new_spanned(&sig, err).to_compile_error().into();
    }

    let fn_name = sig.ident.clone();
    let wasm_ident = format_ident!("__vm_{}", fn_name);

    let gen = quote! {

        #input

        #[cfg(target_family = "wasm")]
        const _: () = {

            #[doc(hidden)]
            #[no_mangle]
            fn #wasm_ident() {
                #fn_name();
            }
        };
    };
    gen.into()
}
