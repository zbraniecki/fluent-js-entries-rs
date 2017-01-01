#![feature(proc_macro)]

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate fluent;

use self::serde::ser::{Serialize, Serializer};
use fluent::syntax::ast;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Resource(pub Vec<Entry>);

impl Serialize for Resource {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(self.0.len())).unwrap();
        for e in &self.0 {
            match e {
                &Entry::Message(ref m) => {
                    try!(serializer.serialize_map_key(&mut map, &m.id));
                    try!(serializer.serialize_map_value(&mut map, &m.value));
                }
            }
        }
        serializer.serialize_map_end(map)
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Entry {
    Message(Message),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub value: Option<Pattern>,
    pub traits: Option<Vec<Member>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Pattern {
    pub elements: Vec<PatternElement>,
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        if self.elements.len() == 1 {
            match self.elements[0] {
                PatternElement::Text(ref t) => serializer.serialize_str(&t),
                _ => panic!(),
            }
        } else {
            panic!();
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternElement {
    Text(String),
    Placeable(Vec<Expression>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub key: String,
    pub value: Pattern,
    pub default: bool,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    EntityReference(Identifier),
}

impl From<ast::Resource> for Resource {
    fn from(r: ast::Resource) -> Resource {
        Resource(r.0
            .into_iter()
            .map(|e| Entry::from(e))
            .collect::<Vec<_>>())
    }
}

impl From<ast::Entry> for Entry {
    fn from(e: ast::Entry) -> Entry {
        match e {
            ast::Entry::Message(m) => {
                Entry::Message(Message {
                    id: m.id,
                    value: Some(Pattern::from(m.value.unwrap())),
                    traits: None,
                })
            }
        }
    }
}

impl From<ast::Identifier> for Identifier {
    fn from(i: ast::Identifier) -> Identifier {
        Identifier(String::from("key2"))
    }
}

impl From<ast::Pattern> for Pattern {
    fn from(p: ast::Pattern) -> Pattern {
        Pattern {
            elements: p.elements
                .into_iter()
                .map(|e| PatternElement::from(e))
                .collect::<Vec<_>>(),
        }
    }
}

impl From<ast::PatternElement> for PatternElement {
    fn from(e: ast::PatternElement) -> PatternElement {
        match e {
            ast::PatternElement::Text(t) => PatternElement::Text(t),
            ast::PatternElement::Placeable(p) => {
                PatternElement::Placeable(p.into_iter()
                    .map(|e| Expression::from(e))
                    .collect::<Vec<_>>())
            }
        }
    }
}

impl From<ast::Expression> for Expression {
    fn from(e: ast::Expression) -> Expression {
        match e {
            ast::Expression::EntityReference(er) => {
                Expression::EntityReference(Identifier::from(er))
            }
        }
    }
}

pub fn serialize_json(res: &Resource) -> String {
    serde_json::to_string_pretty(res).unwrap()
}

pub fn parse(s: &str) -> Result<Resource, fluent::syntax::parser::ParserError> {
    let res = fluent::syntax::parse(s)?;
    Ok(Resource::from(res))
}
