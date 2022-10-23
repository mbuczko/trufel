#[macro_use]
extern crate quote;
extern crate proc_macro;

use std::{fs, path::Path};

use chumsky::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Lit, Meta, MetaNameValue};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Type {
    Typed,
    Untyped,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Arity {
    FetchAll,
    FetchOne,
    FetchOptional,
    FetchMany,
}

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Meta(String, Type, Arity),
    Doc(String),
    Sql(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Query {
    pub name: String,
    pub typ: Type,
    pub arity: Arity,
    pub doc: Option<String>,
    pub sql: String,
}

impl Query {
    fn from(elements: Vec<Element>) -> Self {
        let mut name = String::default();
        let mut doc = None;
        let mut sql = String::default();
        let mut typ = Type::Typed;
        let mut arity = Arity::FetchAll;

        for e in elements {
            match e {
                Element::Meta(n, t, a) => {
                    name = n;
                    typ = t;
                    arity = a;
                }
                Element::Doc(d) => doc = Some(d),
                Element::Sql(s) => sql = s,
            }
        }
        if name.is_empty() {
            panic!(":name attribute is missing or is not a valid identifier. Query: \"{}\"", sql.trim());
        }
        Query {
            name,
            typ,
            arity,
            doc,
            sql,
        }
    }
    pub fn _is_valid(&self) -> bool {
        !self.name.is_empty()
    }
    pub fn build_arity(c: Option<char>) -> Arity {
        if let Some(arity) = c {
            match arity {
                '?' => Arity::FetchOptional,
                '^' => Arity::FetchMany,
                '1' => Arity::FetchOne,
                '*' => Arity::FetchAll,
                _ => Arity::FetchAll,
            }
        } else {
            // default arity = fetch_all
            Arity::FetchAll
        }
    }
    pub fn build_type(t: Option<&'static str>) -> Type {
        if let Some(query_type) = t {
            match query_type {
                "()" => Type::Untyped,
                "<>" => Type::Typed,
                _ => Type::Typed,
            }
        } else {
            // default query type = Typed
            Type::Typed
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
        use futures_core::future::Future;
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
        match q.typ {
            Type::Typed => generate_typed_fn(q, ts),
            Type::Untyped => generate_untyped_fn(q, ts),
        }
    }
}

fn generate_typed_fn(q: Query, ts: &mut TokenStream2) {
    let name = format_ident!("{}", q.name);
    let doc = q.doc.unwrap_or_default();
    let sql = q.sql;

    let bound = quote! { T: Send + Unpin + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + 'e };

    ts.extend(match q.arity {
        Arity::FetchMany => {
            quote! {
                #[doc = #doc]
                async fn #name<'e, T> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> BoxStream<'e, Result<T, sqlx::Error>>
                where #bound {
                    sqlx::query_as_with(#sql, params).fetch(conn)
                }
            }
        },
        Arity::FetchOne => {
            quote! {
                #[doc = #doc]
                async fn #name<'e, T> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> Result<T, sqlx::Error>
                where #bound {
                    sqlx::query_as_with(#sql, params).fetch_one(conn).await
                }
            }
        },
        Arity::FetchOptional => {
            quote! {
                #[doc = #doc]
                async fn #name<'e, T> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> Result<Option<T>, sqlx::Error>
                where #bound {
                    sqlx::query_as_with(#sql, params).fetch_optional(conn).await
                }
            }
        },
        Arity::FetchAll => {
            quote! {
                #[doc = #doc]
                async fn #name<'e, T> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> Result<Vec<T>, sqlx::Error>
                where #bound {
                    sqlx::query_as_with(#sql, params).fetch_all(conn).await
                }
            }
        }
    });
}

fn generate_untyped_fn(q: Query, ts: &mut TokenStream2) {
    let name = format_ident!("{}", q.name);
    let doc = q.doc.unwrap_or_default();
    let sql = q.sql;

    ts.extend(match q.arity {
        Arity::FetchMany => {
            quote! {
                #[doc = #doc]
                async fn #name<'e> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> BoxStream<'e, Result<sqlx::postgres::PgRow, sqlx::Error>> {
                    sqlx::query_with(#sql, params).fetch(conn)
                }
            }
        },
        Arity::FetchOne => {
            quote! {
                #[doc = #doc]
                async fn #name<'e> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
                    sqlx::query_with(#sql, params).fetch_one(conn).await
                }
            }
        },
        Arity::FetchOptional => {
            quote! {
                #[doc = #doc]
                async fn #name<'e> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> Result<Option<sqlx::postgres::PgRow>, sqlx::Error> {
                    sqlx::query_with(#sql, params).fetch_optional(conn).await
                }
            }
        },
        Arity::FetchAll => {
            quote! {
                #[doc = #doc]
                async fn #name<'e> (conn: &'e sqlx::Pool<Postgres>, params: PgArguments) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
                    sqlx::query_with(#sql, params).fetch_all(conn).await
                }
            }
        }
    });
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
        .ignore_then(just('*').or(just('?')).or(just('^')).or(just('1')))
        .padded()
        .labelled("arity");

    let typ = just(':')
        .ignore_then(just("<>").or(just("()")))
        .padded()
        .labelled("type");

    let name = comment
        .ignore_then(just(':'))
        .ignore_then(just("name").padded())
        .ignore_then(text::ident())
        .padded()
        .then(typ.or_not())
        .then(arity.or_not())
        .map(|((ident, t), a)| Element::Meta(ident, Query::build_type(t), Query::build_arity(a)))
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
fn parsing_defaults() {
    use crate::Arity;
    use crate::Type;

    let input = r#"
-- :name fetch_users
-- :doc Returns all the users from DB
SELECT user_id, email, name, picture FROM users
"#;

    let queries = parser().parse(input).unwrap();
    assert_eq!(queries.len(), 1);
    assert_eq!(queries[0].name, "fetch_users");
    assert_eq!(
        queries[0].doc,
        Some("Returns all the users from DB".to_string())
    );
    assert_eq!(queries[0].typ, Type::Typed);
    assert_eq!(queries[0].arity, Arity::FetchAll);
}

#[test]
fn parsing_default_type() {
    use crate::Arity;
    use crate::Type;

    let input = r#"
-- :name fetch_users :^
SELECT user_id, email, name, picture FROM users
"#;

    let queries = parser().parse(input).unwrap();
    assert_eq!(queries.len(), 1);
    assert_eq!(queries[0].name, "fetch_users");
    assert_eq!(queries[0].doc, None);
    assert_eq!(queries[0].typ, Type::Typed);
    assert_eq!(queries[0].arity, Arity::FetchMany);
}

#[test]
fn parsing_default_arity() {
    use crate::Arity;
    use crate::Type;

    let input = r#"
-- :name fetch_users :()
SELECT user_id, email, name, picture FROM users
"#;

    let queries = parser().parse(input).unwrap();
    assert_eq!(queries.len(), 1);
    assert_eq!(queries[0].name, "fetch_users");
    assert_eq!(queries[0].doc, None);
    assert_eq!(queries[0].typ, Type::Untyped);
    assert_eq!(queries[0].arity, Arity::FetchAll);
}

#[test]
fn parsing_multiple() {
    let input = r#"
-- :name fetch_users
-- :doc Returns all the users from DB
SELECT user_id, email, name, picture FROM users

-- :name fetch_user_by_id :() :1
-- :doc Fetches user by its identifier
SELECT user_id, email, name, picture
  FROM users
 WHERE user_id = $1

-- :name set_picture :() :1
-- :doc Sets user's picture.
-- Picture is expected to be a valid URL.
UPDATE users
   -- expected URL to the picture
   SET picture = ?
 WHERE user_id = ?

-- :name delete_user :() :1
DELETE FROM users
 WHERE user_id = ?
"#;

    let queries = parser().parse(input).unwrap();
    assert_eq!(queries.len(), 4);

    assert_eq!(queries[0].name, "fetch_users".to_string());
    assert_eq!(queries[0].doc, Some("Returns all the users from DB".to_string()));
    assert_eq!(queries[0].typ, Type::Typed);
    assert_eq!(queries[0].arity, Arity::FetchAll);

    assert_eq!(queries[1].name, "fetch_user_by_id".to_string());
    assert_eq!(queries[1].doc, Some("Fetches user by its identifier".to_string()));
    assert_eq!(queries[1].typ, Type::Untyped);
    assert_eq!(queries[1].arity, Arity::FetchOne);

    assert_eq!(queries[2].name, "set_picture".to_string());
    assert_eq!(queries[2].doc, Some("Sets user's picture.\nPicture is expected to be a valid URL.".to_string()));
    assert_eq!(queries[2].typ, Type::Untyped);
    assert_eq!(queries[2].arity, Arity::FetchOne);

    assert_eq!(queries[3].name, "delete_user".to_string());
    assert_eq!(queries[3].doc, None);
    assert_eq!(queries[3].typ, Type::Untyped);
    assert_eq!(queries[3].arity, Arity::FetchOne);

}

