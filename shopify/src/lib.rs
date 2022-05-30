#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod result;
#[macro_use]
pub mod client;
mod types;

pub mod fulfillment_service;
pub mod inventory;
pub mod session;
pub mod customer;
pub mod customer_saved_search;
pub mod customer_address;
pub mod order;
pub mod context;
pub mod auth;
pub mod pagination;
pub mod product;
pub mod shop;
pub mod variant;
