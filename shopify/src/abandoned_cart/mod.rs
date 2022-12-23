use crate::client::{Client, Method};
use crate::result::*;

mod types;
pub use self::types::*;

request_query! {
    pub struct CheckoutParams {
        pub since_id: Option<String>,
        pub limit: Option<i64>,
        pub created_at_max: Option<String>,
        pub created_at_min: Option<String>,
        pub updated_at_max: Option<String>,
        pub updated_at_min: Option<String>
    }
}

shopify_wrap! {
  pub struct GetAbandonedCheckouts {
    checkouts: Vec<Checkout>,
  }
}

impl Client {
  async fn get_abandoned_checkouts(&self,params: &CheckoutParams) -> ShopifyResult<Vec<Checkout>> {
    let res: GetAbandonedCheckouts = self.request_with_params(
      Method::GET,
      &format!("/admin/api/{}/checkouts.json", self.context.api_version),
      params,
      std::convert::identity,
    ).await?;


    Ok(res.into_inner())
  }
}
