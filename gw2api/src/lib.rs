//! gw2api is a fairly simple wrapper over the Guild Wars 2 API.
//!
//! ```
//! use gw2api::client::Client;
//! use gw2api::v1::Build;
//!
//! let client = Client::new();
//! let build: Build = client.get().unwrap();
//! println!("Current build id: {}", build.id);
//! ```
//!

#![warn(clippy::all)]
//#![warn(missing_docs)]
//#![deny(warnings)]
#![forbid(unsafe_code)]

pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

// Endpoints
pub mod v1;
pub mod v2;

// TODO: Make macros take as an argument if they are going to be using an authenticated request or
// not
// TODO: Finish macros
// TODO: Testing of macros using trybuild (actually unnecessary since they will only be used
// internally and if they work they work)
// #[macro_export]
// macro_rules! impl_endpoint {
//     // Basic endpoint (single path, no ids)
//     ($target:ty, $path:expr $(,$auth:expr, $localised:expr)?) => {
//         impl gw2api_core::endpoint::Endpoint for $target {
//             fn get(client: &gw2api_core::client::Client) -> gw2api_core::client::Result<Self> {
//                 let builder = gw2api_core::client::RequestBuilder::new($path)
//                     $(
//                         .authenticated($auth)
//                         .localised($localised)
//                     )?;
//                 client.request::<Self>(builder)
//             }
//         }
//     };
    // ($target:ty, $path:expr, $id:ty $(,$get_all:tt)?) => {
    //     impl $target {
    //         /// Returns the item with the given `id`.
    //         pub fn get(client: &crate::client::Client, id: $id) -> crate::client::Result<Self> {
    //             let uri = format!("{}?id={}", $path, id);
    //             client.send(crate::RequestBuilder::new(uri))
    //         }
    //
    //         $(
    //
    //         /// Returns all items.
    //         pub fn get_all<C>(client: &C) -> C::Result
    //         where
    //             C: crate::ClientExecutor<Vec<Self>>,
    //         {
    //             stringify!($get_all);
    //
    //             let uri = format!("{}?ids=all", $path);
    //             client.send(crate::RequestBuilder::new(uri))
    //         }
    //
    //         )?
    //
    //         /// Returns a list of all item ids.
    //         ///
    //         /// # Examples
    //         ///
    //         /// Using the [`blocking`] client:
    //         /// ```ignore
    //         /// # use gw2api_rs::Result;
    //         /// # use gw2api_rs::blocking::Client;
    //         /// #
    //         /// # fn main() -> Result<()> {
    //         /// let client = Client::new();
    //         #[doc = concat!("let ids: Vec<", stringify!($id), "> = ", stringify!($target), "::ids(&client)?;")]
    //         /// println!("{:?}", ids);
    //         /// #
    //         /// # Ok(())
    //         /// # }
    //         /// ```
    //         pub fn ids<C>(client: &C) -> C::Result
    //         where
    //             C: crate::ClientExecutor<Vec<$id>>,
    //         {
    //             client.send(crate::RequestBuilder::new($path))
    //         }
    //     }
    // };
// }

// Export macros to the crate scope
// pub use impl_endpoint;
