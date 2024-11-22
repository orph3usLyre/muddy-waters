use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Error, Expr, ExprLit, Ident, Token, Visibility};
use syn::{ItemStatic, LitStr};

use crate::DEFAULT_ENV;

/// Defines whether the key will be embedded in the binary or read at runtime
#[derive(Default)]
pub enum KeyMode {
    #[default]
    Embedded,
    Env(Option<String>),
}

impl Parse for KeyMode {
    fn parse(input: ParseStream) -> Result<Self> {
        let punctuated: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(input)?;
        let mut iter = punctuated.into_iter();

        let Some(first) = iter.next() else {
            return Ok(Self::default());
        };

        let key_mode = match first.value().as_str() {
            "embed" => Self::Embedded,
            "env" => iter.next().map_or_else(
                || {
                    if let Some(var) = std::env::var_os(DEFAULT_ENV) {
                        if let Ok(var) = var.into_string() {
                            return Self::Env(Some(var));
                        }
                    }
                    Self::Env(None)
                },
                |var| Self::Env(Some(var.value())),
            ),
            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    "Must be one of 'embed', 'env'",
                ))
            }
        };
        Ok(key_mode)
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
        Ok(Self { texts })
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

        // TODO: is it safe to allow mutability?
        if let syn::StaticMutability::Mut(_) = mutability {
            return Err(Error::new(Span::call_site(), "Expression can't be mutable"));
        }
        let Expr::Lit(ExprLit {
            lit: syn::Lit::Str(text),
            ..
        }) = *expr
        else {
            return Err(Error::new(
                Span::call_site(),
                "Expression must be of type string literal",
            ));
        };

        Ok(Self {
            visibility,
            variable_name,
            text,
        })
    }
}
