
use crate::client::{Client, Method};
use crate::result::*;

mod types;
pub use self::types::*;

impl Client {
  pub async fn get_location_list(&self) -> ShopifyResult<Vec<Location>> {
    shopify_wrap! {
      pub struct Res {
        locations: Vec<Location>,
      }
    }

    let res: Res = self
      .request(Method::GET, "/admin/locations.json", std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }

  pub async fn get_inventory_list(
    &self,
    params: &GetInventoryLevelsParams,
  ) -> ShopifyResult<Vec<InventoryLevel>> {
    shopify_wrap! {
      pub struct Res {
        inventory_levels: Vec<InventoryLevel>,
      }
    }

    let res: Res = self
      .request_with_params(
        Method::GET,
        "/admin/inventory_levels.json",
        params,
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  pub async fn get_location(&self, id: i64) -> ShopifyResult<Location> {
    shopify_wrap! {
      pub struct Res {
        location: Location,
      }
    }
    let path = format!("/admin/locations/{}.json", id);
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }
}

request_query! {
  pub struct GetInventoryLevelsParams {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub inventory_item_ids: Option<Vec<i64>>,
    pub location_ids: Option<Vec<i64>>,
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
