use proc_macro2::Span;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{Lit, Token};

use super::arg::Arg;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BindMethodRole {
    ToString,
}

impl BindMethodRole {
    fn try_from(value: String, span: Span) -> std::result::Result<Self, Error> {
        match value.as_str() {
            "to_string" => Ok(Self::ToString),
            _ => Err(Error::new(
                span,
                "expected 'role' bind attribute parameter value: to_string",
            )),
        }
    }
}

/// Arguments of the `bind` attribute.
#[derive(Default)]
pub struct BindArgs {
    name: Option<String>,
    role: Option<BindMethodRole>,
}

impl BindArgs {
    /// If exists returns the name of the function used in C Api and Lua FFI,
    /// otherwise Rust method name is used.
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// If true then the function return string representation of the object.
    /// 'tostring' binding will be added to the metatype section of the Lua FFI file.
    pub fn is_to_string(&self) -> bool {
        let Some(ty) = self.role else { return false; };

        ty == BindMethodRole::ToString
    }
}

impl Parse for BindArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let params = input.parse_terminated(Arg::parse, Token![,])?;
        let mut res = Self::default();

        for param in params {
            match param.name.as_str() {
                "name" => {
                    if let Lit::Str(val) = &param.value.lit {
                        res.name = Some(val.value());
                    } else {
                        return Err(Error::new(
                            param.value.span(),
                            "expected 'name' bind attribute parameter as string literal",
                        ));
                    }
                }
                "role" => {
                    if let Lit::Str(val) = &param.value.lit {
                        let ty = BindMethodRole::try_from(val.value(), param.value.span())?;
                        res.role = Some(ty);
                    } else {
                        return Err(Error::new(
                            param.value.span(),
                            "expected 'name' bind attribute parameter as string literal",
                        ));
                    }
                }
                _ => {
                    return Err(Error::new(
                        param.name.span(),
                        format!("expected bind attribute parameter: name, role"),
                    ))
                }
            }
        }

        Ok(res)
    }
}
