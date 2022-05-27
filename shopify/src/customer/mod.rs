use crate::client::{Client, Method};
use crate::order::Order;
use crate::result::*;

mod types;
pub use self::types::*;

impl Client {
  async fn get_customer_list(&self) -> ShopifyResult<Vec<Customer>> {
    shopify_wrap! {
      pub struct Res {
        customers: Vec<Customer>,
      }
    }

    let res: Res = self
      .request(
        Method::GET,
        &format!("/admin/{}/customers.json", self.context.api_version),
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  async fn create_customer(&self, params: &CustomerParams) -> ShopifyResult<Customer> {
    shopify_wrap! {
      pub struct Res {
        customer:Customer,
      }
    }

    let res: Res = self
      .request(
        Method::POST,
        &format!("/admin/{}/customers.json", self.context.api_version),
        |b| b.json(params),
      )
      .await?;
    Ok(res.into_inner())
  }

  async fn get_customer(&self, id: i64) -> ShopifyResult<Customer> {
    shopify_wrap! {
      pub struct Res {
        customer: Customer,
      }
    }
    let path = format!("/admin/{}/customers/{}.json", self.context.api_version, id);
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }

  async fn get_customer_orders(&self,id: i64) -> ShopifyResult<Vec<Order>> {
    shopify_wrap! {
        pub struct Res {
          orders: Vec<Order>,
        }
      }
      let path = format!("/admin/{}/customers/{}/orders.json", self.context.api_version, id);
      let res: Res = self
        .request(Method::GET, &path, std::convert::identity)
        .await?;
      Ok(res.into_inner())
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
