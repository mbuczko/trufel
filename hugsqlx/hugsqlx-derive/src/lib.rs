use proc_macro::TokenStream;

#[proc_macro_derive(HugSql, attributes(queries))]
pub fn hugsqlx(input_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(input_stream).unwrap();
    hugsqlx_analyzer::impl_hug_sqlx(&ast).into()
}
