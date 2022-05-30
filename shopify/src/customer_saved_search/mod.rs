use crate::{
  client::{Client, Method},
  customer::Customer,
  result::ShopifyResult,
};

mod types;
pub use self::types::*;

impl Client {
  async fn get_saved_searches(
    &self,
    params: SavedParams,
  ) -> ShopifyResult<Vec<CustomerSavedSearchResponse>> {
    shopify_wrap! {
      pub struct Res {
        customer_saved_search: Vec<CustomerSavedSearchResponse>,
      }
    }

    let res: Res = self
      .request_with_params(
        Method::GET,
        &format!(
          "/admin/{}/customer_saved_searches.json",
          self.context.api_version
        ),
        &params,
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  async fn update_saved_search(
    &self,
    customer_saved_search_id: i64,
    customer_saved_search: CustomerSavedSearchResponse,
  ) -> ShopifyResult<CustomerSavedSearchResponse> {
    shopify_wrap! {
      pub struct Res {
        customer_saved_search: CustomerSavedSearchResponse,
      }

      pub struct Body {
        customer_saved_search: CustomerSavedSearchResponse
      }
    }

    let res: Res = self
      .request(
        Method::PUT,
        &format!(
          "/admin/{}/customer_saved_searches/{}.json",
          self.context.api_version, customer_saved_search_id
        ),
        |b| {
          b.json(&Body {
            customer_saved_search,
          })
        },
      )
      .await?;
    Ok(res.into_inner())
  }

  async fn create_saved_search(
    &self,
    customer_id: i64,
    customer_saved_search: SavedSearchArg,
  ) -> ShopifyResult<CustomerSavedSearchResponse> {
    shopify_wrap! {
      pub struct Res {
        customer_saved_search: CustomerSavedSearchResponse,
      }

      pub struct Body {
        customer_saved_search: SavedSearchArg
      }
    }

    let res: Res = self
      .request(
        Method::POST,
        &format!(
          "/admin/{}/customer_saved_searches.json",
          self.context.api_version
        ),
        |b| {
          b.json(&Body {
            customer_saved_search,
          })
        },
      )
      .await?;
    Ok(res.into_inner())
  }

  async fn saved_search_by_customer(
    &self,
    customer_saved_search_id: i64,
    params: SavedCustomerParams,
  ) -> ShopifyResult<Vec<Customer>> {
    shopify_wrap! {
      pub struct Res {
        customers: Vec<Customer>,
      }
    }

    let res: Res = self
      .request_with_params(
        Method::GET,
        &format!(
          "/admin/{}/customer_saved_searches/{}/customers.json",
          self.context.api_version, customer_saved_search_id
        ),
        &params,
        std::convert::identity,
      )
      .await?;
    Ok(res.into_inner())
  }

  async fn saved_search_count(
    &self,
    params: SavedSearchCountParams,
  ) -> ShopifyResult<SavedSearchCount> {
    shopify_wrap! {
      SavedSearchCount,
      pub struct Res {
        count: i64,
      }
    }

    let res: Res = self
      .request_with_params(
        Method::GET,
        &format!(
          "/admin/{}/customer_saved_searches/count.json",
          self.context.api_version
        ),
        &params,
        std::convert::identity,
      )
      .await?;
    Ok(res.into())
  }

  async fn delete_saved_search(&self, customer_saved_search_id: i64) -> ShopifyResult<()> {
    let _ = self
      .request(
        Method::DELETE,
        &format!(
          "/admin/{}/customer_saved_searches/{}.json",
          self.context.api_version, customer_saved_search_id
        ),
        std::convert::identity,
      )
      .await?;
    Ok(())
  }

  async fn get_saved_search(
    &self,
    customer_saved_search_id: i64,
  ) -> ShopifyResult<CustomerSavedSearchResponse> {
    shopify_wrap! {
      pub struct Res {
        customer_saved_search: CustomerSavedSearchResponse,
      }
    }
    let path = format!(
      "/admin/{}/customer_saved_searches/{}.json",
      self.context.api_version, customer_saved_search_id
    );
    let res: Res = self
      .request(Method::GET, &path, std::convert::identity)
      .await?;
    Ok(res.into_inner())
  }
}

request_query! {
    pub struct SavedParams {
        pub fields: Option<String>,
        pub limit: Option<i64>,
        pub since_id: Option<String>
    }

    pub struct SavedCustomerParams {
        pub fields: Option<String>,
        pub limit: Option<i64>,
        pub order: Option<String>
    }

    pub struct SavedSearchCountParams {
        pub since_id: Option<String>
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
