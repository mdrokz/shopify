use schemars::JsonSchema;


#[derive(Debug, Default, Serialize, Clone)]
pub struct NewFulfillment {
  tracking_company: Option<String>,
  tracking_number: Option<String>,
  tracking_numbers: Option<Vec<String>>,
  tracking_url: Option<String>,
  tracking_urls: Option<Vec<String>>,
  notify_customer: Option<bool>,
  line_items: Vec<Item>,
  location_id: Option<i64>,
}

#[derive(Debug, Default, Serialize, Clone)]
struct Item {
  id: i64,
  quantity: Option<i64>,
}

impl NewFulfillment {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn location_id(&mut self, id: i64) -> &mut Self {
    self.location_id = Some(id);
    self
  }

  pub fn tracking_number<T: Into<String>>(&mut self, value: T) -> &mut Self {
    let value: String = value.into();
    self.tracking_number = Some(value.clone());
    self.tracking_numbers = Some(vec![value]);
    self
  }

  pub fn tracking_numbers<T>(&mut self, values: T) -> &mut Self
  where
    T: IntoIterator,
    <T as IntoIterator>::Item: Into<String>,
  {
    self.tracking_number = None;
    self.tracking_numbers = Some(values.into_iter().map(Into::into).collect());
    self
  }

  pub fn tracking_company<T: Into<String>>(&mut self, value: T) -> &mut Self {
    self.tracking_company = Some(value.into());
    self
  }

  pub fn tracking_url<T: Into<String>>(&mut self, value: T) -> &mut Self {
    self.tracking_url = Some(value.into());
    self
  }

  pub fn tracking_urls<T>(&mut self, values: T) -> &mut Self
  where
    T: IntoIterator,
    <T as IntoIterator>::Item: Into<String>,
  {
    self.tracking_url = None;
    self.tracking_urls = Some(values.into_iter().map(Into::into).collect());
    self
  }

  pub fn notify_customer(&mut self, value: bool) -> &mut Self {
    self.notify_customer = Some(value);
    self
  }

  pub fn add_item(&mut self, id: i64, quantity: Option<i64>) -> &mut Self {
    match self.line_items.iter().position(|i| i.id == id) {
      Some(pos) => self.line_items[pos].quantity = quantity,
      None => {
        self.line_items.push(Item {
          id: id,
          quantity: quantity,
        });
      }
    };
    self
  }
}

#[derive(Debug, Serialize, Deserialize,Clone,Default, JsonSchema)]
pub struct Fulfillment {
    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "line_items")]
    pub line_items: Vec<LineItem>,

    #[serde(rename = "location_id")]
    pub location_id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "notify_customer")]
    pub notify_customer: bool,

    #[serde(rename = "order_id")]
    pub order_id: i64,

    #[serde(rename = "origin_address")]
    pub origin_address: Vec<OriginAddress>,

    #[serde(rename = "receipt")]
    pub receipt: Receipt,

    #[serde(rename = "service")]
    pub service: String,

    #[serde(rename = "shipment_status")]
    pub shipment_status: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "tracking_company")]
    pub tracking_company: String,

    #[serde(rename = "tracking_numbers")]
    pub tracking_numbers: Vec<String>,

    #[serde(rename = "tracking_urls")]
    pub tracking_urls: Vec<String>,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "variant_inventory_management")]
    pub variant_inventory_management: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default, JsonSchema)]
pub struct LineItem {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "variant_id")]
    pub variant_id: i64,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "grams")]
    pub grams: i64,

    #[serde(rename = "sku")]
    pub sku: String,

    #[serde(rename = "variant_title")]
    pub variant_title: String,

    #[serde(rename = "vendor")]
    pub vendor: Option<serde_json::Value>,

    #[serde(rename = "fulfillment_service")]
    pub fulfillment_service: String,

    #[serde(rename = "product_id")]
    pub product_id: i64,

    #[serde(rename = "requires_shipping")]
    pub requires_shipping: bool,

    #[serde(rename = "taxable")]
    pub taxable: bool,

    #[serde(rename = "gift_card")]
    pub gift_card: bool,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "variant_inventory_management")]
    pub variant_inventory_management: String,

    #[serde(rename = "properties")]
    pub properties: Vec<Option<serde_json::Value>>,

    #[serde(rename = "product_exists")]
    pub product_exists: bool,

    #[serde(rename = "fulfillable_quantity")]
    pub fulfillable_quantity: i64,

    #[serde(rename = "total_discount")]
    pub total_discount: String,

    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: Option<serde_json::Value>,

    #[serde(rename = "fulfillment_line_item_id")]
    pub fulfillment_line_item_id: i64,

    #[serde(rename = "tax_lines")]
    pub tax_lines: Vec<Option<serde_json::Value>>,

    #[serde(rename = "duties")]
    pub duties: Vec<Duty>,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default, JsonSchema)]
pub struct Duty {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "harmonized_system_code")]
    pub harmonized_system_code: String,

    #[serde(rename = "country_code_of_origin")]
    pub country_code_of_origin: String,

    #[serde(rename = "shop_money")]
    pub shop_money: Money,

    #[serde(rename = "presentment_money")]
    pub presentment_money: Money,

    #[serde(rename = "tax_lines")]
    pub tax_lines: Vec<TaxLine>,

    #[serde(rename = "admin_graphql_api_id")]
    pub admin_graphql_api_id: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default, JsonSchema)]
pub struct Money {
    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "currency_code")]
    pub currency_code: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default, JsonSchema)]
pub struct TaxLine {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "rate")]
    pub rate: f64,

    #[serde(rename = "price_set")]
    pub price_set: PriceSet,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default, JsonSchema)]
pub struct PriceSet {
    #[serde(rename = "shop_money")]
    pub shop_money: Money,

    #[serde(rename = "presentment_money")]
    pub presentment_money: Money,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default, JsonSchema)]
pub struct OriginAddress {
    #[serde(rename = "address1")]
    pub address1: String,

    #[serde(rename = "address2")]
    pub address2: String,

    #[serde(rename = "city")]
    pub city: String,

    #[serde(rename = "country_code")]
    pub country_code: String,

    #[serde(rename = "province_code")]
    pub province_code: String,

    #[serde(rename = "zip")]
    pub zip: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default, JsonSchema)]
pub struct Receipt {
    #[serde(rename = "testcase")]
    pub testcase: bool,

    #[serde(rename = "authorization")]
    pub authorization: String,
}