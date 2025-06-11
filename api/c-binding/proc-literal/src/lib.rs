// Licensed under the Apache-2.0 license
use core::panic;

use caliptra_api::mailbox::CommandId;
use proc_macro::{Literal, TokenStream, TokenTree};
use syn::{parse::Parse, parse_macro_input, Error, ExprPath, Ident};

struct CommandIdConstInput {
    const_ident: Ident,
}
impl Parse for CommandIdConstInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr: ExprPath = input.parse()?;
        let mut path = expr.path.segments.iter();
        let const_ident = path.next_back().unwrap().clone();
        let struct_ident = path.next_back().unwrap();
        if struct_ident.ident != Ident::new("CommandId", struct_ident.ident.span()) {
            return Err(Error::new(
                struct_ident.ident.span(),
                "expected `CommandId` const",
            ));
        }
        Ok(CommandIdConstInput {
            const_ident: const_ident.ident,
        })
    }
}

#[proc_macro]
pub fn mailbox_command_id_literal(input: TokenStream) -> TokenStream {
    let command_id = parse_macro_input!(input as CommandIdConstInput);
    let as_string = command_id.const_ident.to_string();
    match as_string.as_str() {
        "FIRMWARE_LOAD" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::FIRMWARE_LOAD.0)).into()
        }
        "GET_IDEV_CERT" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::GET_IDEV_CERT.0)).into()
        }
        "GET_IDEV_INFO" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::GET_IDEV_INFO.0)).into()
        }
        "POPULATE_IDEV_CERT" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::POPULATE_IDEV_CERT.0)).into()
        }
        "GET_LDEV_CERT" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::GET_LDEV_CERT.0)).into()
        }
        "GET_FMC_ALIAS_CERT" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::GET_FMC_ALIAS_CERT.0)).into()
        }
        "GET_RT_ALIAS_CERT" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::GET_RT_ALIAS_CERT.0)).into()
        }
        "ECDSA384_VERIFY" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::ECDSA384_VERIFY.0)).into()
        }
        "LMS_VERIFY" => TokenTree::from(Literal::u32_unsuffixed(CommandId::LMS_VERIFY.0)).into(),
        "STASH_MEASUREMENT" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::STASH_MEASUREMENT.0)).into()
        }
        "INVOKE_DPE" => TokenTree::from(Literal::u32_unsuffixed(CommandId::INVOKE_DPE.0)).into(),
        "DISABLE_ATTESTATION" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::DISABLE_ATTESTATION.0)).into()
        }
        "FW_INFO" => TokenTree::from(Literal::u32_unsuffixed(CommandId::FW_INFO.0)).into(),
        "DPE_TAG_TCI" => TokenTree::from(Literal::u32_unsuffixed(CommandId::DPE_TAG_TCI.0)).into(),
        "DPE_GET_TAGGED_TCI" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::DPE_GET_TAGGED_TCI.0)).into()
        }
        "INCREMENT_PCR_RESET_COUNTER" => TokenTree::from(Literal::u32_unsuffixed(
            CommandId::INCREMENT_PCR_RESET_COUNTER.0,
        ))
        .into(),
        "QUOTE_PCRS" => TokenTree::from(Literal::u32_unsuffixed(CommandId::QUOTE_PCRS.0)).into(),
        "EXTEND_PCR" => TokenTree::from(Literal::u32_unsuffixed(CommandId::EXTEND_PCR.0)).into(),
        "ADD_SUBJECT_ALT_NAME" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::ADD_SUBJECT_ALT_NAME.0)).into()
        }
        "CERTIFY_KEY_EXTENDED" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::CERTIFY_KEY_EXTENDED.0)).into()
        }
        "VERSION" => TokenTree::from(Literal::u32_unsuffixed(CommandId::VERSION.0)).into(),
        "SELF_TEST_START" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::SELF_TEST_START.0)).into()
        }
        "SELF_TEST_GET_RESULTS" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::SELF_TEST_GET_RESULTS.0)).into()
        }
        "SHUTDOWN" => TokenTree::from(Literal::u32_unsuffixed(CommandId::SHUTDOWN.0)).into(),
        "CAPABILITIES" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::CAPABILITIES.0)).into()
        }
        "SET_AUTH_MANIFEST" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::SET_AUTH_MANIFEST.0)).into()
        }
        "AUTHORIZE_AND_STASH" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::AUTHORIZE_AND_STASH.0)).into()
        }
        "GET_IDEV_CSR" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::GET_IDEV_CSR.0)).into()
        }
        "GET_FMC_ALIAS_CSR" => {
            TokenTree::from(Literal::u32_unsuffixed(CommandId::GET_FMC_ALIAS_CSR.0)).into()
        }
        "SIGN_WITH_EXPORTED_ECDSA" => TokenTree::from(Literal::u32_unsuffixed(
            CommandId::SIGN_WITH_EXPORTED_ECDSA.0,
        ))
        .into(),
        "REVOKE_EXPORTED_CDI_HANDLE" => TokenTree::from(Literal::u32_unsuffixed(
            CommandId::REVOKE_EXPORTED_CDI_HANDLE.0,
        ))
        .into(),
        _ => {
            panic!("`{}` is not a valid `CommandId`", &as_string)
        }
    }
}
