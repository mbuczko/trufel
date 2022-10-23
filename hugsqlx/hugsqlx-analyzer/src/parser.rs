use chumsky::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Typed,
    Untyped,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arity {
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
pub struct Query {
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

fn query_parser() -> impl Parser<char, Vec<Query>, Error = Simple<char>> {
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

pub fn parse_queries(input: String) -> Result<Vec<Query>, Vec<Simple<char>>> {
    query_parser().parse(input)
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

