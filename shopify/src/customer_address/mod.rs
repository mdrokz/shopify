use crate::client::{Client, Method};
use crate::order::Order;
use crate::result::*;

mod types;
pub use self::types::*;

impl Client {
  async fn get_address(&self,address_id: i64,customer_id: i64) -> ShopifyResult<CustomerAddress> {
    shopify_wrap! {
      pub struct Res {
        customer_address: CustomerAddress,
      }
    }

    let res: Res = self
      .request(
        Method::GET,
        &format!("/admin/{}/customers/{}/addresses/{}.json",customer_id,address_id, self.context.api_version),
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  async fn create_address(&self,customer_id: i64,address: CustomerAddress) -> ShopifyResult<CustomerAddress> {
    shopify_wrap! {
      pub struct Res {
        customer_address: CustomerAddress,
      }
    }

    let res: Res = self
      .request(
        Method::POST,
        &format!("/admin/{}/customers/{}/addresses.json",customer_id, self.context.api_version),
        |b| b.json(address),
      )
      .await?;
    Ok(res.into_inner())
  }


  async fn get_addresses(&self, customer_id: i64) -> ShopifyResult<Vec<CustomerAddress>> {
    shopify_wrap! {
      pub struct Res {
        addresses: Vec<CustomerAddress>,
      }
    }
    let path = format!("/admin/{}/customers/{}/addresses.json", self.context.api_version, id);
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
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
