use my_http_utils::macros::MyHttpObjectStructure;
use serde::{Deserialize, Serialize};

/// The product scopes of the env as MyLogger server lists them: the default scope first, then
/// the ones set, then the other ones there are records of
#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct ProductScopesHttpModel {
    pub default_scope: String,
    pub scopes: Vec<ProductScopeHttpModel>,
}

/// `applications` - the regular expressions of the application names which go to the scope
#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct ProductScopeHttpModel {
    pub name: String,
    pub applications: Vec<String>,
}
