use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize,Clone,JsonSchema)]
pub struct CustomerDataRequest {
    #[serde(rename = "shop_id")]
    pub shop_id: i64,

    #[serde(rename = "shop_domain")]
    pub shop_domain: String,

    #[serde(rename = "orders_requested")]
    pub orders_requested: Vec<i64>,

    #[serde(rename = "customer")]
    pub customer: Customer,

    #[serde(rename = "data_request")]
    pub data_request: DataRequest,
}

#[derive(Debug, Serialize, Deserialize,Clone,JsonSchema)]
pub struct CustomerDataRedact {
    #[serde(rename = "shop_id")]
    pub shop_id: i64,

    #[serde(rename = "shop_domain")]
    pub shop_domain: String,

    #[serde(rename = "orders_to_redact")]
    pub orders_to_redact: Vec<i64>,

    #[serde(rename = "customer")]
    pub customer: Customer,
}

#[derive(Debug, Serialize, Deserialize,Clone,JsonSchema)]
pub struct ShopRedact {
    #[serde(rename = "shop_id")]
    pub shop_id: i64,

    #[serde(rename = "shop_domain")]
    pub shop_domain: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,JsonSchema)]
pub struct Customer {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "phone")]
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,JsonSchema)]
pub struct DataRequest {
    #[serde(rename = "id")]
    pub id: i64,
}
