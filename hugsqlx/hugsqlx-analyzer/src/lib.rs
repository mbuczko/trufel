extern crate proc_macro;

mod parser;

use parser::{Query, Type, Arity};
use std::{fs, path::Path};
use quote::{format_ident, quote};
use proc_macro2::TokenStream as TokenStream2;
use syn::{Lit, Meta, MetaNameValue};

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

pub fn impl_hug_sqlx(ast: &syn::DeriveInput) -> TokenStream2 {
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
        if let Ok(input) = fs::read_to_string(f) {
            match parser::parse_queries(input) {
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
