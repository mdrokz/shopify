use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]

pub struct SavedSearchCount {
    pub count: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedSearchArg {
    #[serde(rename = "customer_saved_search")]
    pub customer_saved_search: CustomerSavedSearch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerSavedSearch {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "query")]
    pub query: String,
}


#[derive(Debug, Serialize, Deserialize)]
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