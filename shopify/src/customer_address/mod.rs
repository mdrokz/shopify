use crate::client::{Client, Method};
use crate::{map, result::*};
use std::collections::HashMap;

mod types;
pub use self::types::*;

impl Client {
  pub async fn get_address(&self, address_id: i64, customer_id: i64) -> ShopifyResult<CustomerAddress> {
    shopify_wrap! {
      pub struct Res {
        customer_address: CustomerAddress,
      }
    }

    let res: Res = self
      .request(
        Method::GET,
        &format!(
          "/admin/api/{}/customers/{}/addresses/{}.json",
          customer_id, address_id, self.context.api_version
        ),
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn create_address(
    &self,
    customer_id: i64,
    address: CustomerAddress,
  ) -> ShopifyResult<CustomerAddress> {
    shopify_wrap! {
      pub struct Res {
        customer_address: CustomerAddress,
      }

      pub struct Body {
        address: CustomerAddress
      }
    }

    let res: Res = self
      .request(
        Method::POST,
        &format!(
          "/admin/api/{}/customers/{}/addresses.json",
          customer_id, self.context.api_version
        ),
        |b| b.json(&Body { address }),
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn update_address(
    &self,
    address: CustomerAddress,
    address_id: i64,
    customer_id: i64,
  ) -> ShopifyResult<CustomerAddress> {
    shopify_wrap! {
      pub struct Res {
        customer_address: CustomerAddress,
      }

      pub struct Body {
        address: CustomerAddress
      }
    }

    let res: Res = self
      .request(
        Method::PUT,
        &format!(
          "/admin/api/{}/customers/{}/addresses/{}.json",
          customer_id, address_id, self.context.api_version
        ),
        |b| b.json(&Body { address }),
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn set_default_address(
    &self,
    customer_id: i64,
    address_id: i64,
  ) -> ShopifyResult<CustomerAddress> {
    shopify_wrap! {
      pub struct Res {
        customer_address: CustomerAddress,
      }
    }

    let res: Res = self
      .request(
        Method::PUT,
        &format!(
          "/admin/api/{}/customers/{}/addresses/{}/default.json",
          customer_id, address_id, self.context.api_version
        ),
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn delete_address(
    &self,
    customer_id: i64,
    address_id: i64,
  ) -> ShopifyResult<()> {

    let _ = self
      .request(
        Method::DELETE,
        &format!(
          "/admin/api/{}/customers/{}/addresses/{}.json",
          customer_id, address_id, self.context.api_version
        ),
        std::convert::identity,
      )
      .await?;
    Ok(())
  }

  pub async fn bulk_update_address(
    &self,
    customer_id: i64,
    params: BulkUpdateParams,
  ) -> ShopifyResult<()> {
    let address_ids: String = params.address_ids.map_or(String::new(), |v| {
      v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
    });
    let operation = params.operation.map_or(String::new(), |v| v);

    let query = map! {
      "address_ids[]" => address_ids,
      "operation" => operation
    };
    let _ = self
      .request_with_params(
        Method::PUT,
        &format!(
          "/admin/api/{}/customers/{}/addresses/set.json",
          customer_id, self.context.api_version
        ),
        &query,
        std::convert::identity,
      )
      .await?;
    Ok(())
  }

  pub async fn get_addresses(&self, customer_id: i64) -> ShopifyResult<Vec<CustomerAddress>> {
    shopify_wrap! {
      pub struct Res {
        addresses: Vec<CustomerAddress>,
      }
    }
    let path = format!(
      "/admin/api/{}/customers/{}/addresses.json",
      self.context.api_version, customer_id
    );
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }
}

pub struct BulkUpdateParams {
  pub address_ids: Option<Vec<i64>>,
  pub operation: Option<String>,
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
