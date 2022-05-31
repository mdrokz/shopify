use crate::client::{Client, Method};
use crate::pagination::{GetPage, Paginated};
use crate::result::*;
use serde::Serialize;

mod types;
pub use self::types::*;

request_query! {
  pub struct GetProductListParams {
    pub limit: Option<i64>,
    pub since_id: Option<i64>,
    pub fields: Option<Vec<String>>,
  }
}

impl Client {
  pub async fn list_product(
    &self,
    params: &GetProductListParams,
  ) -> ShopifyResult<Paginated<Vec<Product>>> {
    shopify_wrap! {
      pub struct Res {
        products: Vec<Product>,
      }
    }

    let res: Paginated<Res> = self
      .request_with_params_paginated(
        Method::GET,
        &format!("/admin/api/{}/products.json", self.context.api_version),
        params,
        std::convert::identity,
      )
      .await?;
    Ok(res.map(|p| p.into_inner()))
  }

  pub async fn list_product_page(
    &self,
    params: &GetPage,
  ) -> ShopifyResult<Paginated<Vec<Product>>> {
    shopify_wrap! {
      pub struct Res {
        products: Vec<Product>,
      }
    }
    let res: Paginated<Res> = self
      .request_with_params_paginated(
        Method::GET,
        &format!("/admin/api/{}/products.json", self.context.api_version),
        params,
        std::convert::identity,
      )
      .await?;
    Ok(res.map(|p| p.into_inner()))
  }

  pub async fn update_product<V: Serialize>(&self, id: i64, value: V) -> ShopifyResult<Product> {
    shopify_wrap! {
      pub struct Res {
        product: Product,
      }
    }

    let path = format!(
      "/admin/api/{}/products/{}.json",
      self.context.api_version, id
    );
    let res: Res = self
      .request(Method::PUT, &path, move |b| {
        b.json(&json!({
          "product": value,
        }))
      })
      .await?;
    Ok(res.into_inner())
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   const TMP_DIR: &'static str = "../tmp/products";

//   #[test]
//   fn test_get_product_list() {
//     let client = ::client::get_test_client();
//     let mut params = GetProductListParams::default();
//     params.limit = Some(250);
//     client.get_list(&params).unwrap();
//   }

//   #[test]
//   fn test_dump_all_products() {
//     use serde_json::{self, Value};
//     use std::fs::File;

//     shopify_wrap! {
//       pub struct RawProducts {
//         products: Vec<Value>,
//       }
//     }

//     let client = ::client::get_test_client();
//     let mut params = GetProductListParams::default();
//     params.limit = Some(250);
//     let mut page = 1;
//     loop {
//       println!("Downloading page {}", page);

//       let products = client
//         .request_with_params::<_, RawProducts, _>(
//           Method::GET,
//           "/admin/products.json",
//           &params,
//           std::convert::identity,
//         )
//         .unwrap()
//         .into_inner();

//       if products.is_empty() {
//         break;
//       }

//       let id = products
//         .last()
//         .unwrap()
//         .get("id")
//         .unwrap()
//         .as_i64()
//         .unwrap();
//       println!("count = {}, last_id = {}", products.len(), id);

//       params.page = Some(page + 1);

//       let f = File::create(format!("{}/product_{}.json", TMP_DIR, page)).unwrap();
//       serde_json::to_writer_pretty(f, &products).unwrap();

//       page = page + 1;
//       ::std::thread::sleep_ms(500);
//     }
//   }

//   #[test]
//   #[ignore]
//   fn test_deserialize_all_products() {
//     use serde_json::{self, Value};
//     use std::fs;
//     use std::io;
//     use std::io::Write;

//     let mut chunk = 1;
//     loop {
//       let f = match fs::File::open(format!("{}/product_{}.json", TMP_DIR, chunk)) {
//         Ok(f) => f,
//         Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
//           break;
//         }
//         Err(ref err) => panic!("io err: {}", err),
//       };

//       println!("testing chunk {}", chunk);

//       let products: Vec<Value> = serde_json::from_reader(f).unwrap();

//       let total = products.len();
//       for (i, product) in products.into_iter().enumerate() {
//         let id = product.get("id").unwrap().as_i64().unwrap();
//         let as_str = serde_json::to_string_pretty(&product).unwrap();
//         let mut current = fs::File::create(format!("{}/current_product.json", TMP_DIR)).unwrap();
//         write!(&mut current, "{}", &as_str).unwrap();
//         let product: Product = serde_json::from_str(&as_str).unwrap();
//         println!("testing product {}: {} of {}", id, i + 1, total);
//       }

//       chunk = chunk + 1;
//     }
//   }
// }
