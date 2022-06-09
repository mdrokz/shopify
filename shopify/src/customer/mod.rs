use crate::client::{Client, Method};
use crate::order::Order;
use crate::result::*;

mod types;
pub use self::types::*;

impl Client {
  pub async fn get_customer_list(&self) -> ShopifyResult<Vec<Customer>> {
    shopify_wrap! {
      pub struct Res {
        customers: Vec<Customer>,
      }
    }

    let res: Res = self
      .request(
        Method::GET,
        &format!("/admin/api/{}/customers.json", self.context.api_version),
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn create_customer(&self, customer: &CustomerArg) -> ShopifyResult<Customer> {
    shopify_wrap! {
      pub struct Res {
        customer:Customer,
      }

      pub struct Body {
        customer: CustomerArg
      }
    }

    let res: Res = self
      .request(
        Method::POST,
        &format!("/admin/api/{}/customers.json", self.context.api_version),
        |b| b.json(&Body { customer: customer.clone() }),
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn get_customer(&self, id: i64) -> ShopifyResult<Customer> {
    shopify_wrap! {
      pub struct Res {
        customer: Customer,
      }
    }
    let path = format!(
      "/admin/api/{}/customers/{}.json",
      self.context.api_version, id
    );
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }

  pub async fn get_customer_orders(&self, id: i64) -> ShopifyResult<Vec<Order>> {
    shopify_wrap! {
      pub struct Res {
        orders: Vec<Order>,
      }
    }
    let path = format!(
      "/admin/api/{}/customers/{}/orders.json",
      self.context.api_version, id
    );
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }

  pub async fn get_customer_count(&self) -> ShopifyResult<CustomerCount> {
    shopify_wrap! {
      CustomerCount,
      pub struct Res {
        count: i64,
      }
    }
    let path = format!(
      "/admin/api/{}/customers/count.json",
      self.context.api_version
    );
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into())
  }

  pub async fn update_customer(&self, customer: &CustomerArg, id: i64) -> ShopifyResult<Customer> {
    shopify_wrap! {
      pub struct Res {
        customer: Customer,
      }
      pub struct Body {
        customer: CustomerArg
      }
    }
    let path = format!(
      "/admin/api/{}/customers/{}.json",
      self.context.api_version, id
    );
    let res: Res = self
      .request(Method::PUT, &path, |b| b.json(&Body { customer: customer.clone() }))
      .await?;
    Ok(res.into_inner())
  }

  pub async fn search_customer(&self, params: &CustomerParams) -> ShopifyResult<Vec<Customer>> {
    shopify_wrap! {
      pub struct Res {
        customers: Vec<Customer>,
      }
    }
    let path = format!(
      "/admin/api/{}/customers/search.json",
      self.context.api_version
    );
    let res: Res = self
      .request_with_params(Method::PUT, &path, params, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }
}

request_query! {
    pub struct CustomerParams {
        pub fields: Option<String>,
        pub limit: Option<i64>,
        pub order: Option<String>,
        pub query: Option<String>
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
