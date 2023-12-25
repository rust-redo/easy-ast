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

          fn lookup_pos(&self, span: Span) -> ((u32, u32), (Option<usize>, Option<usize>)) {
            let ctx = self.ctx.as_ref().unwrap();
            (
              ctx.source_map.span_to_char_offset(&ctx.source_file, span),
              (ctx.source_file.lookup_line(span.lo), ctx.source_file.lookup_line(span.hi))
            )
          }
      }
  };
  gen.into()
}

