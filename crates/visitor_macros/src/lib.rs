extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(BaseVisitor)]
pub fn base_visitor_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<syn::DeriveInput>(input).unwrap();

  let name = &ast.ident;
  let gen = quote! {
      impl BaseVisitor for #name {
          fn set_ctx(&mut self, ctx: VisitorContext) {
            self.ctx = Some(ctx);
          }
      }
  };
  gen.into()
}

