use crate::client::{Client, Method};
use crate::pagination::{GetPage, Paginated};
use crate::result::*;
use serde::Serialize;

mod types;
pub use self::types::*;

request_query! {
  pub struct GetVariantListParams {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub since_id: Option<i64>,
    pub fields: Option<Vec<String>>,
  }
}

// pub trait ProductVariantApi {
//   fn list(&self, params: &GetVariantListParams) -> ShopifyResult<Paginated<Vec<Variant>>>;
//   fn list_page(&self, params: &GetPage) -> ShopifyResult<Paginated<Vec<Variant>>>;
//   fn update<V: Serialize>(&self, id: i64, value: V) -> ShopifyResult<Variant>;
// }

impl Client {
  pub async fn list_product_variant(
    &self,
    params: &GetVariantListParams,
  ) -> ShopifyResult<Paginated<Vec<Variant>>> {
    shopify_wrap! {
      pub struct Res {
        variants: Vec<Variant>,
      }
    }

    let res: Paginated<Res> = self
      .request_with_params_paginated(
        Method::GET,
        &format!("/admin/api/{}/variants.json", self.context.api_version),
        params,
        std::convert::identity,
      )
      .await?;
    Ok(res.map(|p| p.into_inner()))
  }

  pub async fn list_variant_page(
    &self,
    params: &GetPage,
  ) -> ShopifyResult<Paginated<Vec<Variant>>> {
    shopify_wrap! {
      pub struct Res {
        variants: Vec<Variant>,
      }
    }
    let res: Paginated<Res> = self
      .request_with_params_paginated(
        Method::GET,
        &format!("/admin/api/{}/variants.json", self.context.api_version),
        params,
        std::convert::identity,
      )
      .await?;
    Ok(res.map(|p| p.into_inner()))
  }

  pub async fn create_product_variant(&self,variant: VariantArg) -> ShopifyResult<Variant> {
    shopify_wrap! {
      pub struct Res {
        variant: Variant,
      }

      pub struct Body {
        variant: VariantArg
      }
    }

    let path = format!(
      "/admin/api/{}/variant.json",
      self.context.api_version
    );
    let res: Res = self
      .request(Method::POST, &path, move |b| {
        b.json(&Body {variant: variant.clone()})
      })
      .await?;
    Ok(res.into_inner())
  }

  pub async fn delete_product_variant(&self,product_id: i64) -> ShopifyResult<()> {
    let path = format!(
      "/admin/api/{}/products/{}.json",
      self.context.api_version, product_id
    );
    let _ = self
      .request(Method::DELETE, &path, std::convert::identity)
      .await?;
    Ok(())
  }

  pub async fn delete_variant(&self,product_id: i64,variant_id: i64) -> ShopifyResult<()> {
    let path = format!(
      "/admin/api/{}/products/{}/variants/{}.json",
      self.context.api_version, product_id,variant_id
    );
    let _ = self
      .request(Method::DELETE, &path, std::convert::identity)
      .await?;
    Ok(())
  }

  pub async fn update_product_variant<V: Serialize>(
    &self,
    id: i64,
    value: V,
  ) -> ShopifyResult<Variant> {
    shopify_wrap! {
      pub struct Res {
        variant: Variant,
      }
    }

    let path = format!("/admin/variants/{}.json", id);
    let res: Res = self
      .request(Method::PUT, &path, move |b| {
        b.json(&json!({
          "variant": value,
        }))
      })
      .await?;
    Ok(res.into_inner())
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   const TMP_DIR: &'static str = "../tmp/variants";

//   #[test]
//   fn test_get_variant_list() {
//     let client = ::client::get_test_client();
//     let mut params = GetVariantListParams::default();
//     params.limit = Some(250);
//     client.get_list(&params).unwrap();
//   }

//   #[test]
//   fn test_dump_all_variants() {
//     use serde_json::{self, Value};
//     use std::fs::File;

//     shopify_wrap! {
//       pub struct RawVariants {
//         variants: Vec<Value>,
//       }
//     }

//     let client = ::client::get_test_client();
//     let mut params = GetVariantListParams::default();
//     params.limit = Some(250);
//     let mut page = 1;
//     loop {
//       println!("Downloading page {}", page);

//       let variants = client
//         .request_with_params::<_, RawVariants, _>(
//           Method::GET,
//           "/admin/variants.json",
//           &params,
//           std::convert::identity,
//         )
//         .unwrap()
//         .into_inner();

//       if variants.is_empty() {
//         break;
//       }

//       let id = variants
//         .last()
//         .unwrap()
//         .get("id")
//         .unwrap()
//         .as_i64()
//         .unwrap();
//       println!("count = {}, last_id = {}", variants.len(), id);

//       params.page = Some(page + 1);

//       let f = File::create(format!("{}/variant_{}.json", TMP_DIR, page)).unwrap();
//       serde_json::to_writer_pretty(f, &variants).unwrap();

//       page = page + 1;
//       ::std::thread::sleep_ms(500);
//     }
//   }

//   #[test]
//   // #[ignore]
//   fn test_deserialize_all_variants() {
//     use serde_json::{self, Value};
//     use std::fs;
//     use std::io;
//     use std::io::Write;

//     let mut chunk = 1;
//     loop {
//       let f = match fs::File::open(format!("{}/variant_{}.json", TMP_DIR, chunk)) {
//         Ok(f) => f,
//         Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
//           break;
//         }
//         Err(ref err) => panic!("io err: {}", err),
//       };

//       println!("testing chunk {}", chunk);

//       let variants: Vec<Value> = serde_json::from_reader(f).unwrap();

//       let total = variants.len();
//       for (i, variant) in variants.into_iter().enumerate() {
//         let id = variant.get("id").unwrap().as_i64().unwrap();
//         let as_str = serde_json::to_string_pretty(&variant).unwrap();
//         let mut current = fs::File::create(format!("{}/current_variant.json", TMP_DIR)).unwrap();
//         write!(&mut current, "{}", &as_str).unwrap();
//         let variant: Variant = serde_json::from_str(&as_str).unwrap();
//         println!("testing variant {}: {} of {}", id, i + 1, total);
//       }

//       chunk = chunk + 1;
//     }
//   }
// }
