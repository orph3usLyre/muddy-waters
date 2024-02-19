use std::str::FromStr;

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, ExprLit, Ident, Visibility};
use syn::{ItemStatic, LitStr};

pub(crate) struct KeyMode(Mode);

impl KeyMode {
    /// Returns the given keymode
    pub fn mode(&self) -> &Mode {
        &self.0
    }
}

pub(crate) enum Mode {
    None,
    Embedded,
    Env,
}

impl Parse for KeyMode {
    fn parse(input: ParseStream) -> Result<Self> {
        let keymode = match input.parse::<LitStr>() {
            Ok(mode) => KeyMode(mode.value().parse()?),
            Err(_) => KeyMode(Mode::None),
        };
        // let mode: LitStr = input.parse()?;
        // let mode: Mode = mode.value().parse()?;
        Ok(keymode)
    }
}

impl FromStr for Mode {
    type Err = syn::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let out = match s {
            "embed" => Mode::Embedded,
            "env" => Mode::Env,
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Must be one of 'embed', 'env'",
                ))
            }
        };
        Ok(out)
    }
}

pub struct NonObfuscatedTexts {
    pub texts: Vec<NonObfuscatedText>,
}

pub struct NonObfuscatedText {
    pub visibility: Visibility,
    pub variable_name: Ident,
    pub text: LitStr,
}

impl Parse for NonObfuscatedTexts {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut texts = Vec::new();
        while let Ok(non_obfuscated_text) = input.parse() {
            texts.push(non_obfuscated_text);
        }
        Ok(NonObfuscatedTexts { texts })
    }
}

impl Parse for NonObfuscatedText {
    fn parse(input: ParseStream) -> Result<Self> {
        let ItemStatic {
            vis: visibility,
            mutability,
            ident: variable_name,
            expr,
            ..
        } = input.parse()?;

        if let syn::StaticMutability::Mut(_) = mutability {
            panic!("Expression cannot be mutable");
        }
        let Expr::Lit(ExprLit {
            lit: syn::Lit::Str(text),
            ..
        }) = *expr
        else {
            panic!("Expression must be of type &'static str");
        };

        Ok(NonObfuscatedText {
            visibility,
            variable_name,
            text,
        })
    }
}
