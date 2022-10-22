#[macro_use]
extern crate quote;
extern crate proc_macro;

use std::{fs, path::Path};

use chumsky::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Data, DataStruct, Fields, Lit, Meta, MetaNameValue, Type};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Arity {
    FetchAll,
    FetchOne,
    FetchOptional,
    FetchStream,
}

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Meta(String, Arity),
    Doc(String),
    Sql(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Query {
    pub name: String,
    pub arity: Arity,
    pub doc: Option<String>,
    pub sql: String,
}

impl Query {
    fn from(elements: Vec<Element>) -> Self {
        let mut name = String::default();
        let mut doc = None;
        let mut sql = String::default();
        let mut arity = Arity::FetchAll;

        for e in elements {
            match e {
                Element::Meta(n, a) => {
                    name = n;
                    arity = a;
                },
                Element::Doc(d) => doc = Some(d),
                Element::Sql(s) => sql = s,
            }
        }
        Query {
            name,
            arity,
            doc,
            sql,
        }
    }
    pub fn _is_valid(&self) -> bool {
        !self.name.is_empty()
    }
    pub fn arity_from_char(ch: char) -> Arity {
        match ch {
            '?' => Arity::FetchOptional,
            '!' => Arity::FetchStream,
            '1' => Arity::FetchOne,
            _ => Arity::FetchAll,
        }
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
    let mut queries_paths = find_attribute_values(ast, "queries");
    if queries_paths.len() != 1 {
        panic!(
            "#[derive(HugSql)] must contain one attribute like this #[queries = \"db/queries/\"]"
        );
    }
    let folder_path = queries_paths.remove(0);

    let canonical_folder_path = Path::new(&folder_path)
        .canonicalize()
        .expect("folder path must resolve to an absolute path");
    let canonical_folder_path = canonical_folder_path
        .to_str()
        .expect("absolute folder path must be valid unicode");

    let files = walkdir::WalkDir::new(canonical_folder_path)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(move |e| {
            Some(std::fs::canonicalize(e.path()).expect("Could not get canonical path"))
        });

    let mut fns = TokenStream2::new();

    for f in files {
        if let Ok(data) = fs::read_to_string(f) {
            match parser().parse(data) {
                Ok(ast) => {
                    generate_impl_fns(ast, &mut fns);
                }
                Err(parse_errs) => parse_errs
                    .into_iter()
                    .for_each(|e| println!("Parse error: {}", e)),
            }
        }
    }

    let name = &ast.ident;
    let mut ts = TokenStream2::new();
    ts.extend(quote! {
        use futures_core::stream::BoxStream;
        use sqlx::{database::HasArguments, postgres::{PgPool, Postgres}, Arguments, IntoArguments, Type, Database};

        #[async_trait::async_trait]
        pub trait HugSql<'q> {
            #fns
        }
        impl<'q> HugSql<'q> for #name {
        }
    });
    ts
}

fn generate_impl_fns(queries: Vec<Query>, ts: &mut TokenStream2) {
    for q in queries {
        let name = format_ident!("{}", q.name);
        let doc = q.doc.unwrap_or_default();

        ts.extend(quote! {
            #[doc = #doc]
            async fn #name<'e, T> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> BoxStream<'e, Result<T, sqlx::Error>>
            where
                T: Send + Unpin + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + 'e,
            {
               sqlx::query_as_with("SELECT * FROM users", params).fetch(conn)

            }

        })
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

    let arity = just(':')
        .ignore_then(just('*').or(just('?')).or(just('!')).or(just('1')))
        .labelled("arity");

    let name = comment
        .ignore_then(just(':'))
        .ignore_then(just("name").padded())
        .ignore_then(text::ident())
        .padded()
        .then(arity)
        .map(|(ident, a)| Element::Meta(ident, Query::arity_from_char(a)))
        .labelled("name");

    let doc = comment
        .ignore_then(just(':'))
        .ignore_then(just("doc").padded())
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
        .map(|(v, _)| Element::Doc(v.iter().collect::<String>()))
        .labelled("doc");

    let sql = take_until(name.or(doc).rewind().ignored().or(end()))
        .padded()
        .map(|(v, _)| Element::Sql(v.iter().collect::<String>()))
        .labelled("sql");

    let query = name
        .or(doc)
        .repeated()
        .at_least(1)
        .at_most(2)
        .chain(sql)
        .map(Query::from);

    query.repeated().then_ignore(end())
}

#[test]
fn parsing() {
    let input = r#"
-- :name fetch_user_by_id :1
-- :doc Fetches user by its identifier
-- and that's almost that!
SELECT user_id, email, name, picture FROM users WHERE user_id = $1

-- :name updates_user :*
-- :doc Juhu juhuu!
SELECT user_id, email, name, picture FROM users
"#;

    match parser().parse(input) {
        Ok(ast) => println!("output {:?}", ast),
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }

    assert!(1 == 2);
}
