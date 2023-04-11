use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::collections::HashMap;

use crate::client::{Client, Result};

pub trait Endpoint: DeserializeOwned + PartialEq + Debug {
    fn get(client: &Client) -> Result<Self>;
}

pub trait ParamEndpoint: Endpoint {
    fn get_ids(client: &Client, ids: &Vec<&'static str>) -> Result<Self>;
    fn get_all(client: &Client) -> Result<Self>;
}

pub trait TwoParamEndpoint: Endpoint {
    fn get_ids(client: &Client, ids: &Vec<&'static str>, other_ids: &Vec<&'static str>) -> Result<Self>;
    fn get_all(client: &Client) -> Result<Self>;
}

#[allow(unused)]
pub trait MultiParamEndpoint: Endpoint {
    fn get_ids(client: &Client, ids: &HashMap<&'static str, &'static str>) -> Result<Self>;
    fn get_all(client: &Client) -> Result<Self>;
}
