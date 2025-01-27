use crate::internal::{
    AttributeArgs,
    NestedMeta,
    BRUSH_PREFIX,
};
use proc_macro::TokenStream;
use proc_macro2::{
    TokenStream as TokenStream2,
    TokenTree,
};
use quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    parse_macro_input,
    spanned::Spanned,
    ImplItemMethod,
};

const INSTANCE: &'static str = "__brush_instance_modifier";

pub(crate) fn generate(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    let modifiers = parse_macro_input!(_attrs as AttributeArgs);
    let mut impl_item =
        syn::parse2::<ImplItemMethod>(_input.into()).expect("Can't parse input of `modifiers` macro like a method.");

    if impl_item.sig.inputs.is_empty() {
        return (quote_spanned! {
            impl_item.sig.inputs.span() =>
                compile_error!("Modifiers can only be applied to methods, which have `self` as their first argument. ");
        })
        .into()
    }

    let receiver;
    if let syn::FnArg::Receiver(rec) = impl_item.sig.inputs.first().unwrap() {
        receiver = rec;
    } else {
        return (quote_spanned! {
            impl_item.sig.inputs.first().unwrap().span() =>
                compile_error!("First argument in method must be `self`.");
        })
        .into()
    }

    // We skip every function without body(it means that it contains only `{ ; }`)
    if impl_item.block.to_token_stream().to_string() == "{ ; }" {
        let code = quote! {
            #impl_item
        };
        return code.into()
    }

    let mut block = impl_item.block.clone();
    let mut body_index = 0;

    // Code of each modifier must be added in reverse order
    // Code of first modifier {
    //      Code of second modifier {
    //          Code of third modifier {
    //              ...
    //          }
    //      }
    // }
    for modifier_meta in modifiers.iter().rev() {
        // Replace every `self` with instance variable
        block = replace_self(block);

        // Put the body of original function to local lambda function
        let (final_block, body_ident) = put_into_closure(receiver, block, body_index);
        body_index += 1;

        // It means modifiers without arguments, we can call path method directly.
        match modifier_meta {
            NestedMeta::Path(method) => {
                let stmts = final_block.stmts;
                block = syn::parse2::<syn::Block>(quote! {
                    {
                        #(#stmts)*
                        #method(self, #body_ident)
                    }
                })
                .unwrap();
            }
            NestedMeta::List(meta_list) => {
                let method = meta_list.path.clone();
                let mut cloned_variables_idents = vec![];
                let cloned_variables_definitions = meta_list.nested.iter().map(|nested_meta| {
                    let cloned_ident = format_ident!("{}_cloned_{}", BRUSH_PREFIX, cloned_variables_idents.len());
                    cloned_variables_idents.push(cloned_ident.clone());
                    quote! {
                        let #cloned_ident = #nested_meta.clone();
                    }
                });

                let stmts = final_block.stmts;
                block = syn::parse2::<syn::Block>(quote! {
                    {
                        #(#cloned_variables_definitions)*
                        #(#stmts)*
                        #method(self, #body_ident #(, #cloned_variables_idents )*)
                    }
                })
                .unwrap();
            }
        }
    }

    impl_item.block = block;

    let code = quote! {
        #impl_item
    };

    code.into()
}

fn replace_self(block: syn::Block) -> syn::Block {
    syn::parse2::<syn::Block>(recursive_replace_self(block.to_token_stream())).unwrap()
}

fn recursive_replace_self(token_stream: TokenStream2) -> TokenStream2 {
    token_stream
        .into_iter()
        .map(|token| {
            match &token {
                TokenTree::Ident(ident) => {
                    if ident.to_string() == "self" {
                        TokenTree::Ident(syn::Ident::new(INSTANCE, ident.span()))
                    } else {
                        token
                    }
                }
                TokenTree::Group(group) => {
                    let mut new_group =
                        proc_macro2::Group::new(group.delimiter(), recursive_replace_self(group.stream()));
                    new_group.set_span(group.span());
                    TokenTree::Group(new_group)
                }
                _ => token,
            }
        })
        .collect()
}

fn put_into_closure(receiver: &syn::Receiver, block: syn::Block, index: u8) -> (syn::Block, syn::Ident) {
    let body_ident = format_ident!("{}_body_{}", BRUSH_PREFIX, index);
    let instance_ident = syn::Ident::new(INSTANCE, receiver.span());

    let reference = match receiver.mutability.is_some() {
        true => quote! { &mut },
        false => quote! { & },
    };

    // Put the body of original function to local lambda function
    let final_block = syn::parse2::<syn::Block>(quote! {
        {
            let mut #body_ident = |#instance_ident: #reference Self| #block;
        }
    })
    .unwrap();

    (final_block, body_ident)
}
