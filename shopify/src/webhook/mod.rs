use serde::Serialize;

use crate::client::{Client, Method};
use crate::order::Order;
use crate::result::*;

pub mod types;

pub use self::types::*;

impl Client {
  pub async fn get_webhook_list(&self, params: &WebHookParams) -> ShopifyResult<Vec<Webhook>> {
    shopify_wrap! {
      pub struct Res {
        webhooks: Vec<Webhook>,
      }
    }

    let res: Res = self
      .request_with_params(
        Method::GET,
        &format!("/admin/api/{}/webhooks.json", self.context.api_version),
        params,
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn create_webhook<T>(&self, webhook: &WebHookBody<T>) -> ShopifyResult<Customer>
  where
    T: Serialize,
  {
    shopify_wrap! {
      pub struct Res {
        customer:Customer,
      }
    }

    let res: Res = self
      .request(
        Method::POST,
        &format!("/admin/api/{}/webhooks.json", self.context.api_version),
        |b| b.json(webhook),
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn get_webhook(&self, webhook_id: i64) -> ShopifyResult<Webhook> {
    shopify_wrap! {
      pub struct Res {
        webhook: Webhook,
      }
    }
    let path = format!(
      "/admin/api/{}/webhooks/{}.json",
      self.context.api_version, webhook_id
    );
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }

  pub async fn get_webhook_count(&self) -> ShopifyResult<i64> {
    shopify_wrap! {
      pub struct Res {
        count: i64,
      }
    }
    let path = format!(
      "/admin/api/{}/webhooks/count.json",
      self.context.api_version
    );
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }

  pub async fn update_webhook(
    &self,
    webhook: &WebHookBody,
    webhook_id: i64,
  ) -> ShopifyResult<Webhook> {
    shopify_wrap! {
      pub struct Res {
        webhook: Webhook,
      }
    }
    let path = format!(
      "/admin/api/{}/webhooks/{}.json",
      self.context.api_version, webhook_id
    );
    let res: Res = self
      .request(Method::PUT, &path, |b| b.json(&webhook))
      .await?;
    Ok(res.into_inner())
  }

  pub async fn delete_webhook(&self, webhook_id: i64) -> ShopifyResult<()> {
    let path = format!(
      "/admin/api/{}/webhooks/{}.json",
      self.context.api_version, webhook_id
    );
    let _ = self
      .request(Method::DELETE, &path, std::convert::identity)
      .await?;
    Ok(())
  }
}

request_query! {
    pub struct WebHookParams {
        pub address: Option<String>,
        pub created_at_max: Option<String>,
        pub created_at_min: Option<String>,
        pub fields: Option<Vec<String>>,
        pub limit: Option<i64>,
        pub since_id: Option<i64>,
        pub topic: Option<String>,
        pub updated_at_max: Option<String>,
        pub updated_at_min: Option<String>,
    }
}

#[cfg(test)]
mod tests {
  #[test]
  #[ignore]
  fn test_location_get_list() {
    use super::LocationApi;
    let client = ::client::get_test_client();
    let list = client.get_list().unwrap();
    println!("{:#?}", list);
  }

  #[test]
  #[ignore]
  fn test_inventory_level_get_list() {
    use super::{GetInventoryLevelsParams, InventoryLevelApi};
    let client = ::client::get_test_client();
    let list = client
      .get_list(&GetInventoryLevelsParams {
        inventory_item_ids: Some(vec![2819391175, 5746930631]),
        ..Default::default()
      })
      .unwrap();
    println!("{:#?}", list);
  }
}
