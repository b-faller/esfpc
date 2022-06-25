use serde::{Deserialize, Deserializer};

use crate::lang::ast;
use crate::lang::parser;

#[derive(Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}

#[derive(Deserialize)]
pub struct Rule {
    #[serde(deserialize_with = "deserialize_condition")]
    pub condition: ast::Expr,
    pub action: Action,
}

fn deserialize_condition<'de, D>(deserializer: D) -> Result<ast::Expr, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parser::parse(s).map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Action {
    pub typ: ActionType,
    pub msg: String,
}

impl Action {
    pub fn new(typ: ActionType, msg: String) -> Self {
        Self { typ, msg }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Error,
    Warning,
    Info,
    Success,
}
