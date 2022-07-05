use serde::{Deserialize, Serialize};

#[cfg(feature = "openapi")]
use schemars::JsonSchema;

#[cfg(feature = "sqlx")]
use sqlx::FromRow;


#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]

pub struct SavedSearchCount {
    pub count: i64
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SavedSearchArg {
    #[serde(rename = "customer_saved_search")]
    pub customer_saved_search: CustomerSavedSearch,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CustomerSavedSearch {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "query")]
    pub query: String,
}


#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[cfg_attr(feature = "openapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CustomerSavedSearchResponse {
  #[serde(rename = "created_at")]
  pub created_at: String,

  #[serde(rename = "id")]
  pub id: i64,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "query")]
  pub query: String,

  #[serde(rename = "updated_at")]
  pub updated_at: String,
}