use std::{collections::HashMap, fmt::Display, vec};

#[doc(hidden)]
pub trait ShopifyWrapper<T> {
  fn into_inner(self) -> T;
}

#[doc(hidden)]
pub trait ShopifyRequestQuery {
  fn as_query_pairs(&self) -> Vec<(String, String)>;
}

impl ShopifyRequestQuery for () {
  fn as_query_pairs(&self) -> Vec<(String, String)> {
    vec![]
  }
}

impl<T> ShopifyRequestQuery for Option<T>
where
  T: ShopifyRequestQuery,
{
  fn as_query_pairs(&self) -> Vec<(String, String)> {
    match *self {
      Some(ref v) => v.as_query_pairs(),
      None => vec![],
    }
  }
}

impl<T: Display> ShopifyRequestQuery for HashMap<&str, T>
where
  T: ShopifyRequestQuery,
{
  fn as_query_pairs(&self) -> Vec<(String, String)> {
    let entries = self.iter();

    let mut query_pair = vec![];

    for (key, value) in entries {
      query_pair.push((key.to_string(), format!("{}", value)));
    }

    query_pair
  }
}

impl ShopifyRequestQuery for String {
    fn as_query_pairs(&self) -> Vec<(String, String)> {
        vec![]
    }
}

impl<K, V> ShopifyRequestQuery for (K, V)
where
  K: AsRef<str>,
  V: AsRef<str>,
{
  fn as_query_pairs(&self) -> Vec<(String, String)> {
    vec![(self.0.as_ref().to_owned(), self.1.as_ref().to_owned())]
  }
}
