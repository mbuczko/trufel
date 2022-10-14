#[macro_use]
extern crate quote;
extern crate proc_macro;

use chumsky::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Lit, Meta, MetaNameValue};

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Name(String),
    Doc(String),
    Sql(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Query {
    name: String,
    doc: Option<String>,
    sql: String,
}

impl Query {
    fn from(elements: Vec<Element>) -> Self {
        let mut name = String::default();
        let mut doc = None;
        let mut sql = String::default();

        for e in elements {
            match e {
                Element::Name(n) => name = n,
                Element::Doc(d) => doc = Some(d),
                Element::Sql(s) => sql = s,
            }
        }
        Query { name, doc, sql }
    }
}

/// Find all pairs of the `name = "value"` attribute from the derive input
fn find_attribute_values(ast: &syn::DeriveInput, attr_name: &str) -> Vec<String> {
    ast.attrs
        .iter()
        .filter(|value| value.path.is_ident(attr_name))
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|meta| match meta {
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(val), ..
            }) => Some(val.value()),
            _ => None,
        })
        .collect()
}

fn impl_hug_sql(ast: &syn::DeriveInput) -> TokenStream2 {
    let queries_path = find_attribute_values(ast, "queries");
    if queries_path.len() != 1 {
        panic!(
            "#[derive(HugSql)] must contain one attribute like this #[queries = \"db/queries/\"]"
        );
    }

    let name = &ast.ident;
    quote! {
        use rust_embed::RustEmbed;
        #[derive(RustEmbed)]
        #[folder = "#queries_path"]
        struct #name;

        impl HugSql for #name {

        }
    }
}

#[proc_macro_derive(HugSql, attributes(queries))]
pub fn hug_sql(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_hug_sql(&ast);

    gen.into()
}

fn parser() -> impl Parser<char, Vec<Query>, Error = Simple<char>> {
    let comment = just("--").padded();
    let name = comment
        .ignore_then(just("name"))
        .ignore_then(just(':').padded())
        .ignore_then(text::ident())
        .map(|n| Element::Name(n));

    let doc = comment
        .ignore_then(just("doc"))
        .ignore_then(just(':').padded())
        .ignore_then(take_until(just('\n')))
        .then(
            comment
                .ignore_then(take_until(just('\n')))
                .padded()
                .repeated(),
        )
        .foldl(|(mut v, c), rhs| {
            v.push(c);
            v.extend(rhs.0);
            (v, c)
        })
        .map(|(v, _)| Element::Doc(v.iter().collect::<String>()));

    let sql = take_until(name.or(doc).rewind().ignored().or(end()))
        .map(|(v, _)| Element::Sql(v.iter().collect::<String>()));

    let query = name.or(doc)
        .repeated()
        .at_least(1)
        .at_most(2)
        .chain(sql)
        .map(|elements| Query::from(elements));

    query
        .repeated()
        .then_ignore(end())
}

#[test]
fn parsing() {
    let input = r#"
-- name: fetch_user_by_id
-- doc: Fetches user by its identifier
  SELECT user_id, email, name, picture
  FROM users
 WHERE user_id = $1


  -- name: fetch_user_by_id
UPDATE users
SET name = $1
WHERE user_id = $2


"#;

    match parser().parse(input) {
        Ok(ast) => println!("output {:?}", ast),
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }

    assert!(1 == 2);
}
