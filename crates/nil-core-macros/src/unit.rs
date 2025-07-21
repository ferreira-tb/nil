// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_unit(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_unit {
      use super::#name;
      use crate::military::unit::{Unit, UnitBox, UnitId, UnitKind, UnitStats};

      impl #name {
        pub fn new_boxed() -> UnitBox {
          UnitBox::new(Box::new(Self))
        }
      }

      impl Unit for #name {
        fn id(&self) -> UnitId {
          Self::ID
        }

        fn kind(&self) -> UnitKind {
          Self::KIND
        }

        fn stats(&self) -> UnitStats {
          Self::STATS
        }
      }

      impl From<#name> for UnitId {
        fn from(_: #name) -> UnitId {
          #name::ID
        }
      }
    }
  };

  stream.into()
}
